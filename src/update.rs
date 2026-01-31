use anyhow::{Result, anyhow, bail};
use bytes::Bytes;
use hmac::{Hmac, Mac};
use http::{Response, StatusCode};
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use sha2::Sha256;
use std::{fs, sync::OnceLock};
use tokio::process::Command;

type HmacSha256 = Hmac<Sha256>;

static WEBHOOK_SECRET: OnceLock<String> = OnceLock::new();

pub fn init_secret(path: &str) -> Result<()> {
    let secret = fs::read_to_string(path).map(|s| s.trim().to_string())?;
    WEBHOOK_SECRET
        .set(secret)
        .map_err(|_| anyhow!("Secret already initialized"))?;
    Ok(())
}

pub fn is_enabled() -> bool {
    WEBHOOK_SECRET.get().is_some()
}

pub async fn handle(req: http::Request<Incoming>) -> Response<Full<Bytes>> {
    if !is_enabled() {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::new(Bytes::new()))
            .unwrap();
    }

    let signature = req
        .headers()
        .get("X-Hub-Signature-256")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let Some(signature) = signature else {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Full::new(Bytes::from("Missing signature header")))
            .unwrap();
    };

    let body = match req.into_body().collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(_) => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Full::new(Bytes::from("Failed to read body")))
                .unwrap();
        }
    };

    if !verify_signature(&signature, &body) {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Full::new(Bytes::from("Invalid signature")))
            .unwrap();
    }

    tokio::spawn(async {
        if let Err(e) = execute().await {
            eprintln!("Update failed: {e}");
        }
    });

    Response::builder()
        .status(StatusCode::OK)
        .body(Full::new(Bytes::from("Update initiated")))
        .unwrap()
}

fn verify_signature(sig_header: &str, body: &[u8]) -> bool {
    let Some(secret) = WEBHOOK_SECRET.get() else {
        return false;
    };

    let Some(hex_sig) = sig_header.strip_prefix("sha256=") else {
        return false;
    };

    let Ok(expected_sig) = hex::decode(hex_sig) else {
        return false;
    };

    let Ok(mut mac) = HmacSha256::new_from_slice(secret.as_bytes()) else {
        return false;
    };

    mac.update(body);
    mac.verify_slice(&expected_sig).is_ok()
}

async fn execute() -> Result<()> {
    println!("Starting self-update...");

    println!("  Running git pull...");
    let output = Command::new("git").arg("pull").output().await?;

    if !output.status.success() {
        bail!(
            "git pull failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    println!("  git pull succeeded");

    println!("  Running cargo build --release...");
    let output = Command::new("cargo")
        .args(["build", "--release"])
        .output()
        .await?;

    if !output.status.success() {
        bail!(
            "cargo build failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    println!("  cargo build succeeded");

    println!("Update complete. Exiting for restart...");
    std::process::exit(0);
}
