use anyhow::Result;
use arc_swap::ArcSwap;
use axum::body::Bytes;
use clap::Parser;
use rustc_hash::FxHashMap;
use std::{path::PathBuf, sync::Arc};

mod compiler;
mod routes;
mod sitemap;
mod typst;
mod watcher;
mod web;

pub struct AppState {
    pub routes: FxHashMap<String, compiler::Route>,
    pub sitemap: Bytes,
}

#[derive(Parser, Debug)]
#[command(name = "liamsnow-com")]
struct Args {
    /// Hostname or IP address to bind to
    #[arg(short, long, default_value = "127.0.0.1")]
    address: String,

    /// Port number (1-65535)
    #[arg(short, long, default_value_t = 3232)]
    port: u16,

    /// Watch content directory for changes and rebuild
    #[arg(short, long)]
    watch: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("Generating pages...");
    let state = Arc::new(ArcSwap::from_pointee(build_state().await?));

    if args.watch {
        watcher::spawn(state.clone());
    }

    web::run(state, &args.address, args.port).await
}

pub async fn build_state() -> Result<AppState> {
    // let start = Instant::now();
    let tasks = routes::load("", &PathBuf::from(routes::CONTENT_DIR))?;
    let routes = compiler::compile(tasks).await;
    let sitemap = sitemap::generate(&routes);
    // println!("startup time = {:?}", Instant::now() - start);
    Ok(AppState { routes, sitemap })
}
