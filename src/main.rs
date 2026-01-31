use anyhow::Result;
use arc_swap::ArcSwap;
use clap::Parser;
use rustc_hash::FxHashMap;
use std::{sync::Arc, time::Instant};

mod compiler;
mod indexer;
mod sitemap;
mod typst;
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
    #[arg(short, long, default_value = "127.0.0.1")]
    address: String,

    /// Port number (1-65535)
    #[arg(short, long, default_value_t = 3232)]
    port: u16,

    /// Watch content directory for changes and rebuild
    #[arg(short, long)]
    watch: bool,

    /// Path to the typst binary
    #[arg(short, long, default_value = "typst")]
    typst: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();
    let args = Args::parse();

    typst::set_binary_path(&args.typst);

    println!("Indexing...");
    let index = indexer::index().await?;

    println!("Compiling...");
    let routes = compiler::compile(index).await;

    println!("Building sitemap...");
    let sitemap = sitemap::generate(&routes);

    let state = Arc::new(ArcSwap::from_pointee(AppState { routes, sitemap }));

    if args.watch {
        watcher::spawn(state.clone());
    }

    println!("Startup time = {:?}", Instant::now() - start);

    web::run(state, &args.address, args.port).await
}
