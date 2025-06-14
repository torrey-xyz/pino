//! Procedural macros for the Pino Solana framework.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn, ItemMod};

mod entrypoint;

/// Derive macro for Accounts - generates account context validation
#[proc_macro_derive(Accounts, attributes(account))]
pub fn derive_accounts(input: TokenStream) -> TokenStream {
    let _input = parse_macro_input!(input as DeriveInput);
    // Phase 1: Return empty implementation
    TokenStream::new()
}

/// Attribute macro for instruction handlers
#[proc_macro_attribute]
pub fn instruction(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Phase 1: Return input unchanged
    input
}

/// Attribute macro for account structs
#[proc_macro_attribute]
pub fn account(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Phase 1: Return input unchanged
    input
}

/// Derive macro for PinoAccount - generates zero-copy account wrapper
#[proc_macro_derive(PinoAccount)]
pub fn derive_pino_account(input: TokenStream) -> TokenStream {
    let _input = parse_macro_input!(input as DeriveInput);
    // Phase 1: Return empty implementation
    TokenStream::new()
}

/// Attribute macro for pino_program - generates program entrypoint and routing
#[proc_macro_attribute]
pub fn pino_program(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_clone = input.clone();
    let input_parsed = parse_macro_input!(input as ItemMod);
    
    // Use the entrypoint implementation
    match entrypoint::pino_program_impl(Vec::new(), input_parsed) {
        Ok(tokens) => tokens.into(),
        Err(_) => input_clone, // Fallback to original input on error
    }
}

/// Derive macro for instruction data - generates borsh serialization
#[proc_macro_derive(PinoData)]
pub fn derive_pino_data(input: TokenStream) -> TokenStream {
    let _input = parse_macro_input!(input as DeriveInput);
    // Phase 1: Return empty implementation
    TokenStream::new()
}

/// Attribute macro for processors - generates instruction processor
#[proc_macro_attribute]
pub fn pino_processor(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Phase 1: Return input unchanged
    input
} 