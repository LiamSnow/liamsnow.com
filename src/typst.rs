use crate::routes::CONTENT_DIR;
use anyhow::{Context, Result, bail};
use std::path::PathBuf;
use tokio::process::Command;

pub async fn compile(source_path: &PathBuf) -> Result<String> {
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
        .output()
        .await?;

    if out.stdout.is_empty() {
        bail!("{}", String::from_utf8_lossy(&out.stderr));
    }

    String::from_utf8(out.stdout).context("typst output was not valid UTF-8")
}
