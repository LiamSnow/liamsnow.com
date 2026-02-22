use crate::{WatchArgs, build};
use anyhow::Result;
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher, event::ModifyKind};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, mpsc};
use std::time::Duration;
use std::{mem, thread};
use tungstenite::{Message, WebSocket, accept};

type ClientList = Arc<Mutex<Vec<WebSocket<TcpStream>>>>;

const DEBOUNCE_MS: u64 = 1000;

pub fn run(root: PathBuf, args: WatchArgs) -> Result<()> {
    let addr = SocketAddr::new(args.watch_address, args.watch_port);
    let listener = TcpListener::bind(addr)?;
    let pending: ClientList = Arc::new(Mutex::new(Vec::new()));
    let (tx, rx) = mpsc::channel();

    println!("Watching {} for changes...", root.display());

    spawn_accept_loop(listener, Arc::clone(&pending));
    create_watcher(&root, tx)?;
    spawn_rebuild_loop(rx, pending, root, args);

    Ok(())
}

/// Accepts new websocket connections
fn spawn_accept_loop(listener: TcpListener, pending: ClientList) {
    thread::spawn(move || {
        while let Ok((stream, addr)) = listener.accept() {
            match accept(stream) {
                Ok(ws) => {
                    println!("WebSocket connected: {addr}");
                    pending.lock().unwrap().push(ws);
                }
                Err(e) => eprintln!("Handshake failed for {addr}: {e}"),
            }
        }
    });
}

/// Recursively watches fs `root` directory
fn create_watcher(root: &Path, tx: mpsc::Sender<()>) -> Result<()> {
    let mut watcher = RecommendedWatcher::new(
        move |res: std::result::Result<notify::Event, notify::Error>| {
            if let Ok(notify::Event {
                kind:
                    EventKind::Create(_) | EventKind::Remove(_) | EventKind::Modify(ModifyKind::Data(_)),
                ..
            }) = res
            {
                _ = tx.send(());
            }
        },
        notify::Config::default(),
    )?;
    watcher.watch(root, RecursiveMode::Recursive)?;
    mem::forget(watcher);
    Ok(())
}

/// Rebuilds on fs change & notifies websockets
fn spawn_rebuild_loop(
    rx: mpsc::Receiver<()>,
    pending: ClientList,
    root: PathBuf,
    watch_args: WatchArgs,
) {
    thread::spawn(move || {
        let mut clients: Vec<WebSocket<TcpStream>> = Vec::new();
        while rx.recv().is_ok() {
            while rx.try_recv().is_ok() {}

            println!("Change detected, rebuilding...");

            clients.append(&mut pending.lock().unwrap());

            match build(&root, &watch_args) {
                Err(e) => eprintln!("Rebuild failed: {e}"),
                Ok(()) => {
                    clients.retain_mut(|ws| ws.send(Message::Binary(vec![].into())).is_ok());
                    println!("Notified {} client(s)", clients.len());
                }
            }

            thread::sleep(Duration::from_millis(DEBOUNCE_MS));
            while rx.try_recv().is_ok() {}
        }
        println!("Watcher rebuild loop stopped.");
    });
}
