use crate::compiler::scss::GrassSlotsFs;
use crate::compiler::typst::LiamsWorld;
use crate::indexer::{MetaMap, SlotType, Slots, TypstSlot};
use crate::web::route::Route;
use crate::{RoutingTable, WatchArgs};
use ::typst::foundations::{Dict, Value};
use ::typst::syntax::FileId;
use anyhow::{Context, Result, anyhow, bail};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::ops::Bound;
use std::path::Path;

mod scss;
mod sitemap;
mod typst;

pub fn run(slots: Slots, metamap: MetaMap, root: &Path, watch: &WatchArgs) -> Result<RoutingTable> {
    let mut routing_table = slots
        .par_iter()
        .filter(|(_, slot)| !slot.hidden)
        .map(|(id, slot)| {
            let content = match &slot.ty {
                SlotType::Typst(tslot) => compile_typst(id, tslot, &slots, &metamap, root, watch),
                SlotType::Scss => compile_scss(id, &slots),
                SlotType::Other => Ok(slot.file.to_vec()),
            }
            .with_context(|| format!("{id:?}"))?;

            let route = Route::compile(id, content, &slot.mime, watch.watch)?;
            Ok((slot.url.clone(), route))
        })
        .collect::<Result<RoutingTable>>()?;

    let (url, route) = sitemap::generate(&routing_table, watch)?;
    routing_table.insert(url, route);

    Ok(routing_table)
}

fn compile_scss(id: &FileId, slots: &Slots) -> Result<Vec<u8>> {
    let fs = GrassSlotsFs(slots);
    let opts = grass::Options::default()
        .style(grass::OutputStyle::Compressed)
        .input_syntax(grass::InputSyntax::Scss)
        .fs(&fs);
    let path = id.vpath().as_rooted_path();
    let content = grass::from_path(path, &opts).map_err(|e| anyhow!("{e}"))?;
    Ok(content.into())
}

fn compile_typst(
    id: &FileId,
    tslot: &TypstSlot,
    slots: &Slots,
    metamap: &MetaMap,
    root: &Path,
    watch: &WatchArgs,
) -> Result<Vec<u8>> {
    let mut inputs = Dict::new();

    if let Some(page_meta) = &tslot.page_meta {
        inputs.insert("page".into(), Value::Dict(page_meta.clone()));
    }

    if let Some(queries) = &tslot.queries {
        for (name, query) in queries {
            inputs.insert(name.clone(), eval_query(query, metamap)?);
        }
    }

    let mut world = LiamsWorld::new(*id, slots, inputs, root, watch);
    let doc = world.compile()?;
    let html = world.html(&doc)?;
    Ok(html.into())
}

/// Evaluate a query `/projects/` into an array of the metadata
/// of each page where its url is prefixed `/projects/`
fn eval_query(query: &Value, metamap: &MetaMap) -> Result<Value> {
    let Value::Str(query) = query else {
        bail!("`{query:?}` is not a valid query. Must be a string.");
    };

    let mut end = query.clone().to_string();
    if let Some(last) = end.as_bytes().last().copied() {
        end.pop();
        end.push((last + 1) as char);
    }

    Ok(Value::Array(
        metamap
            .range::<str, _>((
                Bound::Included(query.as_str()),
                Bound::Excluded(end.as_str()),
            ))
            .map(|(_, meta)| Value::Dict(meta.clone()))
            .collect(),
    ))
}
