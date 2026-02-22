use crate::web::route::Route;
use ::typst::comemo;
use anyhow::Result;
use arc_swap::ArcSwap;
use clap::Parser;
use rustc_hash::FxHashMap;
use std::net::IpAddr;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock};
use std::thread::available_parallelism;
use std::time::Instant;

mod compiler;
mod indexer;
mod update;
mod watcher;
mod web;

#[derive(clap::Parser, Debug)]
#[command(name = "liamsnow-com")]
pub struct Args {
    /// Path to content directory
    #[arg(short, long, env = "CONTENT_DIR", default_value = "./content")]
    pub root: PathBuf,

    #[command(flatten)]
    pub web: WebArgs,

    #[command(flatten)]
    pub watch: WatchArgs,

    #[command(flatten)]
    pub update: UpdateArgs,

    /// Number of threads to use. Defaults to number of cores.
    #[arg(short, long, env = "NUM_THREADS")]
    pub threads: Option<usize>,
}

#[derive(clap::Args, Debug, Clone)]
pub struct WebArgs {
    /// Hostname or IP address to bind to
    #[arg(short, long, env = "ADDRESS", default_value = "127.0.0.1")]
    pub address: IpAddr,

    /// Port number (1-65535)
    #[arg(short, long, env = "PORT", default_value_t = 3232)]
    pub port: u16,
}

#[derive(clap::Args, Debug, Clone)]
pub struct WatchArgs {
    /// Watch content directory for changes and rebuild
    #[arg(short, long, env = "WATCH")]
    pub watch: bool,

    /// Watch websocket hostname or IP address to bind to
    #[arg(long, env = "WATCH_ADDRESS", default_value = "127.0.0.1")]
    pub watch_address: IpAddr,

    /// Watch websocket port number (1-65535)
    #[arg(long, env = "WATCH_PORT", default_value_t = 3233)]
    pub watch_port: u16,
}

#[derive(clap::Args, Debug, Clone)]
pub struct UpdateArgs {
    /// Path to file containing GitHub webhook secret
    #[arg(long, env = "GITHUB_SECRET_PATH")]
    pub github_secret: Option<PathBuf>,

    /// Path to the cargo binary
    #[arg(short, long, env = "CARGO", default_value = "cargo")]
    pub cargo: PathBuf,

    /// Path to the git binary
    #[arg(short, long, env = "GIT", default_value = "git")]
    pub git: PathBuf,
}

pub type RoutingTable = FxHashMap<String, Route>;
pub static ROUTING_TABLE: LazyLock<ArcSwap<RoutingTable>> =
    LazyLock::new(|| ArcSwap::from_pointee(RoutingTable::default()));

fn main() -> Result<()> {
    let args = Args::parse();
    update::set_cfg(args.update)?;

    // use all threads for building
    rayon::ThreadPoolBuilder::new().build_global()?;

    build(&args.root, &args.watch)?;

    let mut num_threads = args
        .threads
        .unwrap_or(available_parallelism().map(|n| n.get()).unwrap());

    if args.watch.watch {
        num_threads -= 3;
        if let Err(e) = watcher::run(args.root, args.watch) {
            eprintln!("Watcher error: {e}");
        }
    }

    web::run(args.web, num_threads)
}

fn build(root: &Path, watch: &WatchArgs) -> Result<()> {
    let start = Instant::now();

    println!("Starting Build");

    println!("Indexing...");
    let (slots, metamap) = indexer::run(root)?;

    println!("Compiling...");
    let routing_table = compiler::run(slots, metamap, root, watch)?;

    ROUTING_TABLE.store(Arc::new(routing_table));

    println!("Build done in {:?}", Instant::now() - start);

    comemo::evict(10);

    Ok(())
}
