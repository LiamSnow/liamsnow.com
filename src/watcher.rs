use anyhow::Result;
use arc_swap::ArcSwap;
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher, event::ModifyKind};
use std::{
    net::SocketAddr,
    path::PathBuf,
    sync::{Arc, OnceLock},
    time::Duration,
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{broadcast, mpsc},
    time::sleep,
};
use tokio_tungstenite::tungstenite::Message;

use crate::{AppState, Args, CONTENT_DIR, compiler, indexer, sitemap};

pub static WATCH_ADDR: OnceLock<Option<String>> = OnceLock::new();
const DEBOUNCE_MS: u64 = 100;

pub fn set_watch_addr(args: &Args) {
    let watch_addr = match args.watch {
        true => Some(format!("{}:{}", args.watch_address, args.watch_port)),
        false => None,
    };

    WATCH_ADDR.set(watch_addr).ok();
}

pub async fn spawn(state: Arc<ArcSwap<AppState>>, addr: &str) -> Result<()> {
    let listener = TcpListener::bind(&addr).await?;

    let (on_rebuild_tx, on_rebuild_rx) = broadcast::channel(1);

    // accept ws connections
    tokio::spawn(async move {
        while let Ok((stream, addr)) = listener.accept().await {
            tokio::spawn(conn_handler(on_rebuild_rx.resubscribe(), stream, addr));
        }
    });

    // watch fs and rebuild
    tokio::spawn(async move {
        if let Err(e) = watch(state, on_rebuild_tx).await {
            eprintln!("Watcher error: {e}");
        }
    });

    Ok(())
}

async fn conn_handler(
    mut on_rebuild: broadcast::Receiver<()>,
    raw_stream: TcpStream,
    addr: SocketAddr,
) {
    println!("New websocket connection @ {addr}");
    let mut stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("Websocket connected @ {addr}");

    loop {
        tokio::select! {
            _ = on_rebuild.recv() => {
                if let Err(e) = stream.send(Message::Binary(Bytes::new())).await {
                    eprintln!("Error sending to websocket @ {addr}: {e}");
                    break;
                }
            }
            res = stream.next() => {
                match res {
                    None => break,
                    Some(Result::Err(e)) => {
                        eprintln!("Error from websocket @ {addr}: {e}");
                    }
                    _ => {}
                }
            }
        }
    }

    println!("Websocket disconnected @ {addr}");
}

/// Watch `content/` and trigger rebuilds
async fn watch(state: Arc<ArcSwap<AppState>>, ws_tx: broadcast::Sender<()>) -> Result<()> {
    let (tx, mut rx) = mpsc::channel::<()>(1);

    let mut watcher = RecommendedWatcher::new(
        move |res: Result<notify::Event, notify::Error>| {
            if let Ok(notify::Event {
                kind:
                    EventKind::Create(_) | EventKind::Remove(_) | EventKind::Modify(ModifyKind::Data(_)),
                ..
            }) = res
            {
                let _ = tx.blocking_send(());
            }
        },
        notify::Config::default(),
    )?;

    watcher.watch(
        PathBuf::from(CONTENT_DIR).as_path(),
        RecursiveMode::Recursive,
    )?;

    println!("Watching {} for changes...", CONTENT_DIR);

    while rx.recv().await.is_some() {
        // lil debounce
        sleep(Duration::from_millis(DEBOUNCE_MS)).await;
        while rx.try_recv().is_ok() {}

        println!("Change detected, rebuilding...");
        match rebuild().await {
            Ok(new_state) => {
                state.store(Arc::new(new_state));
                println!("Rebuild complete.");
                if let Ok(n) = ws_tx.send(()) {
                    println!("Notified {n} websockets");
                }
            }
            Err(e) => eprintln!("Rebuild failed: {e}"),
        }

        while rx.try_recv().is_ok() {}
    }

    Ok(())
}

async fn rebuild() -> Result<AppState> {
    let index = indexer::index().await?;
    let routes = compiler::compile(index).await;
    let sitemap = sitemap::generate(&routes);
    Ok(AppState { routes, sitemap })
}
