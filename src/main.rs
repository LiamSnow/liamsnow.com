use anyhow::Result;
use arc_swap::ArcSwap;
use clap::Parser;
use rustc_hash::FxHashMap;
use std::{sync::Arc, time::Instant};

mod compiler;
mod indexer;
mod sitemap;
mod typst;
mod update;
mod watcher;
mod web;

pub const CONTENT_DIR: &str = "./content";

pub struct AppState {
    pub routes: FxHashMap<String, compiler::Route>,
    pub sitemap: compiler::Route,
}

#[derive(Parser, Debug)]
#[command(name = "liamsnow-com")]
struct Args {
    /// Hostname or IP address to bind to
    #[arg(short, long, env = "ADDRESS", default_value = "127.0.0.1")]
    address: String,

    /// Port number (1-65535)
    #[arg(short, long, env = "PORT", default_value_t = 3232)]
    port: u16,

    /// Watch content directory for changes and rebuild
    #[arg(short, long, env = "WATCH")]
    watch: bool,

    /// Watch websocket port number (1-65535)
    #[arg(long, env = "WATCH_PORT", default_value_t = 3233)]
    watch_port: u16,

    /// Watch websocket hostname or IP address to bind to
    #[arg(long, env = "WATCH_ADDRESS", default_value = "127.0.0.1")]
    watch_address: String,

    /// Path to the typst binary
    #[arg(short, long, env = "TYPST", default_value = "typst")]
    typst: String,

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

#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();
    let args = Args::parse();

    typst::set_binary_path(&args.typst);
    update::set_cargo_path(&args.cargo);
    update::set_git_path(&args.git);
    watcher::set_watch_addr(&args);

    if let Some(ref path) = args.github_secret_path {
        update::init_secret(path)?;
        println!("GitHub webhook update enabled");
    }

    println!("Indexing...");
    let index = indexer::index().await?;

    println!("Compiling...");
    let routes = compiler::compile(index).await;

    println!("Building sitemap...");
    let sitemap = sitemap::generate(&routes);

    let state = Arc::new(ArcSwap::from_pointee(AppState { routes, sitemap }));

    if let Some(addr) = watcher::WATCH_ADDR.get().unwrap()
        && let Err(e) = watcher::spawn(state.clone(), addr).await
    {
        eprintln!("Watcher error: {e}");
    }

    println!("Startup time = {:?}", Instant::now() - start);

    web::run(state, &args.address, args.port).await
}
