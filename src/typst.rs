use crate::routes::CONTENT_DIR;
use anyhow::{Context, Result, bail};
use std::path::PathBuf;
use std::sync::OnceLock;
use tokio::process::Command;

static BINARY_PATH: OnceLock<String> = OnceLock::new();

pub fn set_binary_path(path: &str) {
    BINARY_PATH.set(path.to_string()).ok();
}

pub async fn compile(source_path: &PathBuf) -> Result<String> {
    let binary = BINARY_PATH.get().map(|s| s.as_str()).unwrap_or("typst");
    let out = Command::new(binary)
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
