use anyhow::Result;
use arc_swap::ArcSwap;
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher, event::ModifyKind};
use std::{path::PathBuf, sync::Arc};

use crate::{AppState, CONTENT_DIR, compiler, indexer, sitemap};

pub fn spawn(state: Arc<ArcSwap<AppState>>) {
    tokio::spawn(async move {
        if let Err(e) = watch(state).await {
            eprintln!("Watcher error: {e}");
        }
    });
}

async fn watch(state: Arc<ArcSwap<AppState>>) -> Result<()> {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(1);

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
        while rx.try_recv().is_ok() {}

        println!("Change detected, rebuilding...");
        match rebuild().await {
            Ok(new_state) => {
                state.store(Arc::new(new_state));
                println!("Rebuild complete.");
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
