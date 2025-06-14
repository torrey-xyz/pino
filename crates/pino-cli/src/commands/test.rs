//! Test command implementation

use anyhow::Result;
use colored::*;
use std::process::Command;

pub async fn execute(test: Option<&str>) -> Result<()> {
    println!("{} Running tests...", "🧪".green());
    
    let mut cmd = Command::new("cargo");
    cmd.arg("test");
    
    if let Some(test_name) = test {
        cmd.arg(test_name);
        println!("Running specific test: {}", test_name.bold());
    }
    
    let output = cmd.output()?;
    
    if output.status.success() {
        println!("{} Tests passed!", "✅".green());
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("{} Tests failed:", "❌".red());
        println!("{}", stderr);
        anyhow::bail!("Tests failed");
    }
    
    Ok(())
} 