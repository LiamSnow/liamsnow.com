use crate::compiler::scss::GrassSlotsFs;
use crate::compiler::typst::LiamsWorld;
use crate::indexer::{MetaMap, SlotType, Slots, TypstSlot};
use crate::web::route::Route;
use crate::{RoutingTable, WatchArgs};
use ::typst::foundations::{Dict, Value};
use ::typst::syntax::{FileId, VirtualPath};
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
    let css = grass::from_path(path, &opts).map_err(|e| anyhow!("{e}"))?;
    Ok(css.into())
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

    if let Some(css_path) = &tslot.css {
        let vp = VirtualPath::new(css_path);
        let id = FileId::new(None, vp);
        let bytes = compile_scss(&id, slots)?;
        let text = String::from_utf8_lossy(&bytes);
        inputs.insert("css".into(), Value::Str(text.into()));
    }

    let mut world = LiamsWorld::new(*id, slots, inputs, root, watch);
    let doc = world.compile()?;
    let html = world.html(&doc)?;

    let cfg = minify_html::Cfg {
        keep_html_and_head_opening_tags: true,
        minify_css: true,
        minify_js: true,
        preserve_brace_template_syntax: true,
        preserve_chevron_percent_template_syntax: true,
        ..Default::default()
    };
    Ok(minify_html::minify(&html.into_bytes(), &cfg))
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

#[cfg(test)]
mod test {
    use super::*;
    use ::typst::foundations::{Dict, Value};

    fn make_meta(entries: &[(&str, &[(&str, Value)])]) -> MetaMap {
        entries
            .iter()
            .map(|(url, fields)| {
                let dict: Dict = fields.iter().cloned().map(|(k, v)| (k.into(), v)).collect();
                (url.to_string(), dict)
            })
            .collect()
    }

    fn unwrap_array(val: Value) -> Vec<Dict> {
        match val {
            Value::Array(arr) => arr
                .into_iter()
                .map(|v| match v {
                    Value::Dict(d) => d,
                    other => panic!("expected Dict, got {other:?}"),
                })
                .collect(),
            other => panic!("expected Array, got {other:?}"),
        }
    }

    #[test]
    fn query_matches_prefix() {
        let meta = make_meta(&[
            ("/projects/cat", &[("title", Value::Str("Cat".into()))]),
            ("/projects/dog", &[("title", Value::Str("Dog".into()))]),
            ("/blog/post1", &[("title", Value::Str("Post".into()))]),
        ]);

        let result = eval_query(&Value::Str("/projects/".into()), &meta).unwrap();
        let items = unwrap_array(result);

        assert_eq!(items.len(), 2);
        assert_eq!(items[0].get("title").unwrap(), &Value::Str("Cat".into()));
        assert_eq!(items[1].get("title").unwrap(), &Value::Str("Dog".into()));
    }

    #[test]
    fn query_no_matches() {
        let meta = make_meta(&[("/blog/post1", &[("title", Value::Str("Post".into()))])]);

        let result = eval_query(&Value::Str("/projects/".into()), &meta).unwrap();
        let items = unwrap_array(result);

        assert!(items.is_empty());
    }

    #[test]
    fn query_empty_metamap() {
        let meta = MetaMap::new();

        let result = eval_query(&Value::Str("/anything/".into()), &meta).unwrap();
        let items = unwrap_array(result);

        assert!(items.is_empty());
    }

    #[test]
    fn query_no_sibling() {
        let meta = make_meta(&[
            ("/projects/cat", &[("title", Value::Str("Cat".into()))]),
            ("/projectsX/dog", &[("title", Value::Str("Dog".into()))]),
        ]);

        let result = eval_query(&Value::Str("/projects/".into()), &meta).unwrap();
        let items = unwrap_array(result);

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].get("title").unwrap(), &Value::Str("Cat".into()));
    }

    #[test]
    fn query_non_string() {
        let meta = MetaMap::new();

        assert!(eval_query(&Value::Int(42), &meta).is_err());
        assert!(eval_query(&Value::Bool(true), &meta).is_err());
        assert!(eval_query(&Value::None, &meta).is_err());
    }
}
