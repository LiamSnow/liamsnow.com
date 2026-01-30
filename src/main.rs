use anyhow::Result;
use arc_swap::ArcSwap;
use clap::Parser;
use rustc_hash::FxHashMap;
use std::{path::PathBuf, sync::Arc, time::Instant};

mod compiler;
mod routes;
mod sitemap;
mod typst;
mod watcher;
mod web;

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
    let args = Args::parse();

    typst::set_binary_path(&args.typst);

    let state = Arc::new(ArcSwap::from_pointee(build_state().await?));

    if args.watch {
        watcher::spawn(state.clone());
    }

    web::run(state, &args.address, args.port).await
}

pub async fn build_state() -> Result<AppState> {
    let start = Instant::now();
    println!("Indexing routes..");
    let tasks = routes::load("", &PathBuf::from(routes::CONTENT_DIR))?;
    println!("Compiling routes..");
    let routes = compiler::compile(tasks).await;
    println!("Building sitemap..");
    let sitemap = sitemap::generate(&routes);
    println!("Startup time = {:?}", Instant::now() - start);
    Ok(AppState { routes, sitemap })
}
