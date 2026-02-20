use ::typst::comemo;
use anyhow::Result;
use arc_swap::ArcSwap;
use clap::Parser;
use rustc_hash::FxHashMap;
use std::{
    path::PathBuf,
    sync::{Arc, LazyLock, OnceLock},
    time::Instant,
};

use crate::compiler::route::Route;

mod compiler;
mod indexer;
mod typst;
mod update;
mod watcher;
mod web;

#[derive(Parser, Debug, Default)]
#[command(name = "liamsnow-com")]
pub struct Args {
    /// Hostname or IP address to bind to
    #[arg(short, long, env = "ADDRESS", default_value = "127.0.0.1")]
    address: String,

    /// Port number (1-65535)
    #[arg(short, long, env = "PORT", default_value_t = 3232)]
    port: u16,

    /// Path to content directory
    #[arg(short, long, env = "CONTENT_DIR", default_value = "./content")]
    root: PathBuf,

    /// Watch content directory for changes and rebuild
    #[arg(short, long, env = "WATCH")]
    watch: bool,

    /// Watch websocket port number (1-65535)
    #[arg(long, env = "WATCH_PORT", default_value_t = 3233)]
    watch_port: u16,

    /// Watch websocket hostname or IP address to bind to
    #[arg(long, env = "WATCH_ADDRESS", default_value = "127.0.0.1")]
    watch_address: String,

    /// Path to file containing GitHub webhook secret
    #[arg(long, env = "GITHUB_SECRET_PATH")]
    github_secret_path: Option<String>,

    /// Path to the cargo binary
    #[arg(short, long, env = "CARGO", default_value = "cargo")]
    cargo: String,

    /// Path to the git binary
    #[arg(short, long, env = "GIT", default_value = "git")]
    git: String,
}

pub type RoutingTable = FxHashMap<String, Route>;
pub static ROUTING_TABLE: LazyLock<ArcSwap<RoutingTable>> =
    LazyLock::new(|| ArcSwap::from_pointee(RoutingTable::default()));
pub static CONFIG: OnceLock<Args> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(ref path) = args.github_secret_path {
        update::set_secret(path)?;
        println!("GitHub webhook update enabled");
    }

    CONFIG.set(args).expect("CONFIG already set");

    build().await?;

    if CONFIG.get().unwrap().watch
        && let Err(e) = watcher::spawn().await
    {
        eprintln!("Watcher error: {e}");
    }

    web::run().await
}

async fn build() -> Result<()> {
    let start = Instant::now();

    println!("Starting Build");

    println!("Indexing...");
    let slots = indexer::run().await?;

    println!("Compiling...");
    let routing_table = compiler::run(slots).await?;

    ROUTING_TABLE.store(Arc::new(routing_table));

    println!("Build done in {:?}", Instant::now() - start);

    comemo::evict(10);

    Ok(())
}
