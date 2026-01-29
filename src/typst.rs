use crate::CONTENT_DIR;
use anyhow::{Result, bail};
use std::{path::PathBuf, process::Command};

pub fn compile(source_path: &PathBuf) -> Result<String> {
    let out = Command::new("typst")
        .arg("compile")
        .arg(source_path)
        .arg("--features")
        .arg("html")
        .arg("--format")
        .arg("html")
        .arg("--root")
        .arg(CONTENT_DIR)
        .arg("-")
        .output()?;

    if out.stdout.is_empty() {
        println!("{}", String::from_utf8_lossy(&out.stderr));
        bail!("Failed to compile {}", source_path.to_string_lossy());
    }

    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}
