//! # Hello World - Pino Framework Example
//!
//! This example demonstrates how to create a simple Solana program using
//! the Pino framework built on Pinocchio for maximum CU efficiency.

use pino::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use bytemuck::{Pod, Zeroable};

// Declare the program ID
declare_id!("HeLLo1111111111111111111111111111111111111");

/// Program instructions
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub enum HelloInstruction {
    /// Initialize a greeting account
    Initialize { greeting: String },
    /// Update the greeting
    UpdateGreeting { new_greeting: String },
    /// Say hello (read-only operation)
    SayHello,
}

/// Account data structure for storing greetings
#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
pub struct GreetingAccount {
    /// Whether the account is initialized
    pub is_initialized: u8,
    /// Length of the greeting string
    pub greeting_len: u32,
    /// Reserved space for future use
    pub reserved: [u8; 3],
    // Greeting string follows after this struct
}

impl GreetingAccount {
    pub const LEN: usize = core::mem::size_of::<Self>();
    pub const MAX_GREETING_LEN: usize = 100;
    pub const TOTAL_SIZE: usize = Self::LEN + Self::MAX_GREETING_LEN;

    pub fn is_initialized(&self) -> bool {
        self.is_initialized != 0
    }

    pub fn set_initialized(&mut self) {
        self.is_initialized = 1;
    }
}

/// Account validation for Initialize instruction
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = GreetingAccount::TOTAL_SIZE)]
    pub greeting_account: Account<'info, GreetingAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Account validation for UpdateGreeting instruction
#[derive(Accounts)]
pub struct UpdateGreeting<'info> {
    #[account(mut)]
    pub greeting_account: Account<'info, GreetingAccount>,
    pub user: Signer<'info>,
}

/// Account validation for SayHello instruction
#[derive(Accounts)]
pub struct SayHello<'info> {
    pub greeting_account: Account<'info, GreetingAccount>,
}

/// Implement the instruction processor
impl ProgramInstruction for HelloInstruction {
    fn process<'info>(
        &self,
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        _instruction_data: &[u8],
    ) -> ProgramResult {
        match self {
            HelloInstruction::Initialize { greeting } => {
                msg!("Instruction: Initialize");
                let mut accounts_iter = accounts.iter();
                let ctx = parse_accounts::<Initialize>(program_id, &mut accounts_iter, _instruction_data)?;
                initialize(ctx, greeting.clone())
            }
            HelloInstruction::UpdateGreeting { new_greeting } => {
                msg!("Instruction: UpdateGreeting");
                let mut accounts_iter = accounts.iter();
                let ctx = parse_accounts::<UpdateGreeting>(program_id, &mut accounts_iter, _instruction_data)?;
                update_greeting(ctx, new_greeting.clone())
            }
            HelloInstruction::SayHello => {
                msg!("Instruction: SayHello");
                let mut accounts_iter = accounts.iter();
                let ctx = parse_accounts::<SayHello>(program_id, &mut accounts_iter, _instruction_data)?;
                say_hello(ctx)
            }
        }
    }
}

/// Initialize a greeting account
pub fn initialize(ctx: Context<Initialize>, greeting: String) -> ProgramResult {
    require!(
        greeting.len() <= GreetingAccount::MAX_GREETING_LEN,
        PinoError::InvalidInstructionData
    );

    let greeting_account = &ctx.accounts.greeting_account;
    
    // Initialize the account header
    let mut account_data = greeting_account.load_mut()?;
    require!(
        !account_data.is_initialized(),
        PinoError::AccountAlreadyInitialized
    );

    account_data.set_initialized();
    account_data.greeting_len = greeting.len() as u32;

    // Write the greeting string after the account header
    let account_info = greeting_account.info();
    let data = unsafe { account_info.borrow_mut_data_unchecked() };
    let greeting_bytes = greeting.as_bytes();
    
    data[GreetingAccount::LEN..GreetingAccount::LEN + greeting_bytes.len()]
        .copy_from_slice(greeting_bytes);

    msg!("Greeting account initialized with: {}", greeting);
    Ok(())
}

/// Update the greeting in an existing account
pub fn update_greeting(ctx: Context<UpdateGreeting>, new_greeting: String) -> ProgramResult {
    require!(
        new_greeting.len() <= GreetingAccount::MAX_GREETING_LEN,
        PinoError::InvalidInstructionData
    );

    let greeting_account = &ctx.accounts.greeting_account;
    
    // Update the account header
    let mut account_data = greeting_account.load_mut()?;
    require!(
        account_data.is_initialized(),
        PinoError::AccountNotInitialized
    );

    account_data.greeting_len = new_greeting.len() as u32;

    // Update the greeting string
    let account_info = greeting_account.info();
    let data = unsafe { account_info.borrow_mut_data_unchecked() };
    let greeting_bytes = new_greeting.as_bytes();
    
    // Clear old greeting
    data[GreetingAccount::LEN..GreetingAccount::LEN + GreetingAccount::MAX_GREETING_LEN]
        .fill(0);
    
    // Write new greeting
    data[GreetingAccount::LEN..GreetingAccount::LEN + greeting_bytes.len()]
        .copy_from_slice(greeting_bytes);

    msg!("Greeting updated to: {}", new_greeting);
    Ok(())
}

/// Say hello by reading and logging the greeting
pub fn say_hello(ctx: Context<SayHello>) -> ProgramResult {
    let greeting_account = &ctx.accounts.greeting_account;
    
    let account_data = greeting_account.load()?;
    require!(
        account_data.is_initialized(),
        PinoError::AccountNotInitialized
    );

    // Read the greeting string
    let account_info = greeting_account.info();
    let data = unsafe { account_info.borrow_data_unchecked() };
    let greeting_len = account_data.greeting_len as usize;
    
    let greeting_bytes = &data[GreetingAccount::LEN..GreetingAccount::LEN + greeting_len];
    let greeting = core::str::from_utf8(greeting_bytes)
        .map_err(|_| PinoError::InvalidInstructionData)?;

    msg!("Hello! The greeting is: {}", greeting);
    Ok(())
}

// Generate the program entrypoint using Pino's efficient entrypoint
pino_entrypoint!(HelloInstruction); 