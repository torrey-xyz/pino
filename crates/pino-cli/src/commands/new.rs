//! Create new Pino project command

use anyhow::{Result, Context};
use std::fs;
use std::path::Path;
use colored::*;

pub async fn execute(name: &str, template: &str) -> Result<()> {
    println!("{} Creating new Pino project: {}", "✨".green(), name.bold());
    
    let project_path = Path::new(name);
    
    if project_path.exists() {
        anyhow::bail!("Directory '{}' already exists", name);
    }
    
    // Create project directory
    fs::create_dir_all(project_path)
        .with_context(|| format!("Failed to create directory '{}'", name))?;
    
    // Create basic project structure
    create_project_structure(project_path, template)
        .with_context(|| "Failed to create project structure")?;
    
    println!("{} Project '{}' created successfully!", "✅".green(), name.bold());
    println!();
    println!("Next steps:");
    println!("  cd {}", name);
    println!("  pino build");
    println!("  pino test");
    
    Ok(())
}

fn create_project_structure(project_path: &Path, template: &str) -> Result<()> {
    // Create src directory
    let src_path = project_path.join("src");
    fs::create_dir_all(&src_path)?;
    
    // Create Cargo.toml
    let cargo_toml_content = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
pino-core = {{ path = "../../crates/pino-core" }}
pino-macros = {{ path = "../../crates/pino-macros" }}
pinocchio = {{ path = "../../pinocchio/sdk/pinocchio" }}
borsh = "1.0"
bytemuck = "1.20"

[lib]
crate-type = ["cdylib", "lib"]
"#, project_path.file_name().unwrap().to_string_lossy());
    
    fs::write(project_path.join("Cargo.toml"), cargo_toml_content)?;
    
    // Create lib.rs based on template
    let lib_rs_content = match template {
        "basic" => create_basic_template(),
        "token" => create_token_template(),
        _ => create_basic_template(),
    };
    
    fs::write(src_path.join("lib.rs"), lib_rs_content)?;
    
    // Create README.md
    let readme_content = format!(r#"# {}

A Pino Solana program.

## Building

```bash
pino build
```

## Testing

```bash
pino test
```

## Deploying

```bash
pino deploy
```
"#, project_path.file_name().unwrap().to_string_lossy());
    
    fs::write(project_path.join("README.md"), readme_content)?;
    
    Ok(())
}

fn create_basic_template() -> String {
    r#"use pino_core::prelude::*;

#[pino_program]
pub mod program {
    use super::*;

    #[instruction]
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        msg!("Hello from Pino!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
"#.to_string()
}

fn create_token_template() -> String {
    r#"use pino_core::prelude::*;

#[pino_program]
pub mod token_program {
    use super::*;

    #[instruction]
    pub fn initialize_mint(ctx: Context<InitializeMint>) -> ProgramResult {
        msg!("Initializing mint");
        Ok(())
    }

    #[instruction]
    pub fn mint_to(ctx: Context<MintTo>, amount: u64) -> ProgramResult {
        msg!("Minting {} tokens", amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMint {}

#[derive(Accounts)]
pub struct MintTo {}
"#.to_string()
} 