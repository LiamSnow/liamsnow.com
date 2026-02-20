use crate::RoutingTable;
use crate::compiler::route::Route;
use crate::indexer::Slots;
use crate::typst::LiamsWorld;
use anyhow::{Result, bail};
use mime_guess::mime;
use rustc_hash::FxBuildHasher;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::task::JoinSet;
use typst::syntax::FileId;

pub mod route;
mod scss;
mod sitemap;
mod r#static;

pub async fn run(slots: Slots) -> Result<RoutingTable> {
    let mut tasks = JoinSet::new();
    let slots = Arc::new(slots);

    for id in slots.keys().cloned() {
        if slots[&id].is_hidden {
            continue;
        }

        let slots = slots.clone();
        tasks.spawn(async move { compile_one(id, slots) });
    }

    let mut routing_table = HashMap::with_capacity_and_hasher(tasks.len(), FxBuildHasher);

    while let Some(result) = tasks.join_next().await {
        let (url, route) = result??;
        routing_table.insert(url, route);
    }

    let (url, route) = sitemap::generate(&routing_table);
    routing_table.insert(url, route);

    Ok(routing_table)
}

fn compile_one(id: FileId, slots: Arc<Slots>) -> Result<(String, Route)> {
    let slot = &slots[&id];
    let url = slot.url.clone();

    let should_compress = matches!(
        slot.ext.as_ref(),
        "typ" | "scss" | "js" | "txt" | "md" | "csv"
    );

    let result = match slot.ext.as_ref() {
        "typ" => compile_one_typst(id, slots),
        "scss" => scss::compile(slot, &slots),
        _ => r#static::compile(slot),
    };

    match result {
        Ok(route) => Ok((url, route)),
        Err(e) => {
            bail!("{id:?}: {e}");
        }
    }
}

fn compile_one_typst(id: FileId, slots: Arc<Slots>) -> Result<Route> {
    let slot = &slots[&id];
    let inputs = slot.inputs.as_ref().unwrap().clone();
    let lib = Some(crate::typst::library(inputs));
    let mut world = LiamsWorld::new(id, slots, lib);
    let doc = world.compile()?;
    let html = world.html(&doc)?;
    Ok(Route::from_string(html, mime::TEXT_HTML_UTF_8, None))
}
