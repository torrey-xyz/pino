//! Build command implementation

use anyhow::Result;
use colored::*;
use std::process::Command;

pub async fn execute(release: bool) -> Result<()> {
    println!("{} Building Pino program...", "🔨".blue());
    
    let mut cmd = Command::new("cargo");
    cmd.arg("build");
    
    if release {
        cmd.arg("--release");
        println!("Building in release mode");
    }
    
    let output = cmd.output()?;
    
    if output.status.success() {
        println!("{} Build completed successfully!", "✅".green());
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("{} Build failed:", "❌".red());
        println!("{}", stderr);
        anyhow::bail!("Build failed");
    }
    
    Ok(())
} 