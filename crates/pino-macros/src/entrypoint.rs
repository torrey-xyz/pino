//! Program entrypoint macro implementation.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemMod, Result};

/// Implementation of the pino_program macro
pub fn pino_program_impl(_args: Vec<syn::Meta>, input: ItemMod) -> Result<TokenStream> {
    let mod_name = &input.ident;
    let mod_content = input.content.as_ref().map(|(_, items)| items);

    // Extract function handlers
    let mut handlers = Vec::new();
    if let Some(items) = mod_content {
        for item in items {
            if let syn::Item::Fn(func) = item {
                // Check if function has #[instruction] attribute
                for attr in &func.attrs {
                    if attr.path().is_ident("instruction") {
                        handlers.push(&func.sig.ident);
                        break;
                    }
                }
            }
        }
    }

    // Generate entrypoint
    let entrypoint_name = quote::format_ident!("{}_entrypoint", mod_name);
    
    Ok(quote! {
        #input

        /// Program entrypoint
        #[no_mangle]
        pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
            let (program_id, accounts, instruction_data) = 
                unsafe { ::pino_core::pinocchio::entrypoint::parse_input(input) };
            
            match process_instruction(&program_id, accounts, instruction_data) {
                Ok(()) => ::pino_core::SUCCESS,
                Err(_) => 1, // Error code
            }
        }

        /// Process instruction dispatcher
        pub fn process_instruction(
            program_id: &::pino_core::Pubkey,
            accounts: &[::pino_core::AccountInfo],
            instruction_data: &[u8],
        ) -> ::pino_core::ProgramResult {
            // For now, return success - proper routing would be implemented here
            Ok(())
        }
    })
} 