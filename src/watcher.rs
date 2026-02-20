use crate::{CONFIG, build};
use anyhow::Result;
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher, event::ModifyKind};
use std::{net::SocketAddr, path::Path, time::Duration};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{broadcast, mpsc},
    time::sleep,
};
use tokio_tungstenite::tungstenite::Message;

const DEBOUNCE_MS: u64 = 1000;

pub async fn spawn() -> Result<()> {
    let cfg = CONFIG.get().unwrap();
    let addr = format!("{}:{}", cfg.watch_address, cfg.watch_port);
    let listener = TcpListener::bind(&addr).await?;

    let (on_rebuild_tx, on_rebuild_rx) = broadcast::channel(1);

    // accept ws connections
    tokio::spawn(async move {
        while let Ok((stream, addr)) = listener.accept().await {
            tokio::spawn(ws_handler(on_rebuild_rx.resubscribe(), stream, addr));
        }
    });

    // watch fs and rebuild
    tokio::spawn(async move {
        if let Err(e) = watch(on_rebuild_tx, &cfg.root).await {
            eprintln!("Watcher error: {e}");
        }
    });

    Ok(())
}

async fn ws_handler(
    mut on_rebuild: broadcast::Receiver<()>,
    raw_stream: TcpStream,
    addr: SocketAddr,
) {
    let mut stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");

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
}

/// Watch `content/` and trigger rebuilds
async fn watch(ws_tx: broadcast::Sender<()>, content_dir: &Path) -> Result<()> {
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

    watcher.watch(content_dir, RecursiveMode::Recursive)?;

    println!("Watching {} for changes...", content_dir.display());

    while rx.recv().await.is_some() {
        println!("Change detected, rebuilding...");

        if let Err(e) = build().await {
            eprintln!("Rebuild failed: {e}");
        } else if let Ok(n) = ws_tx.send(()) {
            println!("Notified {n} websockets");
        }

        sleep(Duration::from_millis(DEBOUNCE_MS)).await;
        while rx.try_recv().is_ok() {}
    }

    Ok(())
}
