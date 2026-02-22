use crate::UpdateArgs;
use anyhow::{Context, Result, bail};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::path::PathBuf;
use std::process::Command;
use std::{fs, sync::OnceLock};

pub const GH_HEADER: &str = "X-Hub-Signature-256";
pub static SECRET: OnceLock<String> = OnceLock::new();
pub static CARGO: OnceLock<PathBuf> = OnceLock::new();
pub static GIT: OnceLock<PathBuf> = OnceLock::new();

type HmacSha256 = Hmac<Sha256>;

pub fn set_cfg(cfg: UpdateArgs) -> Result<()> {
    let Some(sec_path) = cfg.github_secret else {
        println!("GitHub webhook disabled. Path was not supplied");
        return Ok(());
    };

    let secret = fs::read_to_string(sec_path)
        .map(|s| s.trim().to_string())
        .context("Reading GitHub webhook secret file")?;

    SECRET.set(secret).unwrap();
    CARGO.set(cfg.cargo).unwrap();
    GIT.set(cfg.git).unwrap();

    println!("GitHub webhook update enabled");

    Ok(())
}

pub fn verify(sig: &[u8], body: Vec<u8>) -> bool {
    let sig = String::from_utf8_lossy(sig);

    let Some(secret) = SECRET.get() else {
        return false;
    };

    let Some(hex_sig) = sig.strip_prefix("sha256=") else {
        return false;
    };

    let Ok(expected_sig) = hex::decode(hex_sig) else {
        return false;
    };

    let Ok(mut mac) = HmacSha256::new_from_slice(secret.as_bytes()) else {
        return false;
    };

    mac.update(&body);
    mac.verify_slice(&expected_sig).is_ok()
}

pub fn run() -> Result<()> {
    println!("Starting self-update...");

    println!("  Pulling changes...");
    let output = Command::new(GIT.get().unwrap()).arg("pull").output()?;

    if !output.status.success() {
        bail!(
            "git pull failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    println!("  Building...");
    let output = Command::new(CARGO.get().unwrap())
        .args(["build", "--release"])
        .output()?;

    if !output.status.success() {
        bail!(
            "cargo build failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    println!("Update complete. Exiting for restart...");
    std::process::exit(0);
}
