//! Utility functions for Pino CLI

use anyhow::Result;
use std::path::Path;
use std::process::Command;

/// Check if we're in a Pino project directory
pub fn is_pino_project() -> bool {
    Path::new("Cargo.toml").exists() && 
    std::fs::read_to_string("Cargo.toml")
        .map(|content| content.contains("pino-core"))
        .unwrap_or(false)
}

/// Execute a command and return the output
pub fn execute_command(program: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(program)
        .args(args)
        .output()?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Command failed: {}", stderr);
    }
}

/// Get the current project name from Cargo.toml
pub fn get_project_name() -> Result<String> {
    let cargo_toml = std::fs::read_to_string("Cargo.toml")?;
    
    for line in cargo_toml.lines() {
        if line.trim().starts_with("name") {
            if let Some(name) = line.split('=').nth(1) {
                return Ok(name.trim().trim_matches('"').to_string());
            }
        }
    }
    
    anyhow::bail!("Could not find project name in Cargo.toml");
}

/// Spinner utility for long-running operations
pub struct Spinner {
    message: String,
}

impl Spinner {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
    
    pub fn start(&self) {
        println!("⏳ {}", self.message);
    }
    
    pub fn finish(&self, success: bool) {
        if success {
            println!("✅ {} completed", self.message);
        } else {
            println!("❌ {} failed", self.message);
        }
    }
} 