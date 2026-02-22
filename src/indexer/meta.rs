use anyhow::{Context, Result, bail, ensure};
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::collections::HashMap;
use typst::foundations::{Array, Dict, Value};
use typst::syntax::ast::{self, Arg, ArrayItem, DictItem, Expr, Markup};

pub const PAGE_KEY: &str = "page";
pub const QUERY_KEY: &str = "query";

/// A somewhat hacky way to get around using typst introspection/querying
/// which requires the entire file to be compiled and then queryed.
///
/// This is not an evaluation so variables are not allowed.
///
/// **Example Input**:
/// ```typst
/// #metadata((
///   title: "igloo",
///   desc: "A secure, fast, & intuitive smart home platform",
///   started: "2025-01-01",
///   ended: "Now",
///   lang: "Rust",
///   links: (
///     ("Homepage", "https://igloo.rs"),
///     ("GitHub", "https://github.com/liamsnow/igloo"),
///   ),
///   homepage: true,
///   query: ("/blog/igloo/",),
/// )) <page>
/// ```
pub fn parse(src: &str) -> Result<FxHashMap<String, Dict>> {
    let root = typst::syntax::parse(src);
    let mut exprs = root
        .cast::<Markup>()
        .context("failed to cast to Markup")?
        .exprs();

    let mut results = HashMap::with_capacity_and_hasher(3, FxBuildHasher);

    while let Some(expr) = exprs.next() {
        let Expr::FuncCall(call) = expr else {
            continue;
        };

        if !matches!(call.callee(), Expr::Ident(ident) if ident.get() == "metadata") {
            continue;
        }

        let Some(Arg::Pos(Expr::Dict(dict))) = call.args().items().next() else {
            bail!("expected `#metadata(..)` to contain a dictionary");
        };

        ensure!(
            matches!(exprs.next(), Some(Expr::Space(_))),
            "expected space after `#metadata(())`"
        );

        let Some(Expr::Label(label)) = exprs.next() else {
            bail!("expected space and `<label>` after `#metadata(())`");
        };

        let result = results
            .entry(label.get().to_string())
            .or_insert_with(Dict::new);
        parse_dict(dict, result)?;
    }

    Ok(results)
}

/// Parse an ast::Dict into an existing Dict
fn parse_dict(input: ast::Dict, out: &mut Dict) -> Result<()> {
    for item in input.items() {
        match item {
            DictItem::Named(named) => {
                let key = named.name().get().clone().into();
                let value = parse_expr(named.expr())?;
                out.insert(key, value);
            }
            DictItem::Keyed(keyed) => match keyed.key() {
                Expr::Str(key) => {
                    let key = key.get().into();
                    let value = parse_expr(keyed.expr())?;
                    out.insert(key, value);
                }
                expr => {
                    bail!("expected dictionary key to be string, found `{expr:?}`")
                }
            },
            DictItem::Spread(_) => {
                bail!("unexpected spread `..things` item in dictionary")
            }
        }
    }
    Ok(())
}

fn parse_expr(expr: Expr<'_>) -> Result<Value> {
    Ok(match expr {
        Expr::Str(v) => Value::Str(v.get().into()),
        Expr::Int(v) => Value::Int(v.get()),
        Expr::Float(v) => Value::Float(v.get()),
        Expr::Bool(v) => Value::Bool(v.get()),
        Expr::Numeric(v) => Value::numeric(v.get()),
        Expr::Array(d) => Value::Array(parse_array(d)?),
        Expr::Dict(d) => {
            let mut dict = Dict::new();
            parse_dict(d, &mut dict)?;
            Value::Dict(dict)
        }
        v => bail!("unexpected value `{v:?}`"),
    })
}

fn parse_array(array: ast::Array) -> Result<Array> {
    let mut result = Array::new();
    for item in array.items() {
        match item {
            ArrayItem::Pos(expr) => {
                result.push(parse_expr(expr)?);
            }
            ArrayItem::Spread(_) => {
                bail!("unexpected spread `..things` item in array")
            }
        }
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    use crate::indexer::meta::{PAGE_KEY, QUERY_KEY};
    use typst::foundations::Value;

    #[test]
    fn full() {
        let src = r#"
        #metadata((
          title: "TITLE",
          my_bool: true,
          my_int: 10,
          my_array: (1, 2
          , 3, 4),
          my_dict: (
              key: "value"
          )
        )) <page>

        #metadata((query1: "/blog/")) <query>
        #metadata((query2: "/projects/")) <query>

        = My content!
        
        "#;

        let res = super::parse(src).unwrap();

        assert_eq!(
            res,
            [
                (
                    PAGE_KEY.into(),
                    [
                        ("title".into(), Value::Str("TITLE".into())),
                        ("my_bool".into(), Value::Bool(true)),
                        ("my_int".into(), Value::Int(10)),
                        (
                            "my_array".into(),
                            Value::Array(
                                [Value::Int(1), Value::Int(2), Value::Int(3), Value::Int(4)]
                                    .into_iter()
                                    .collect(),
                            ),
                        ),
                        (
                            "my_dict".into(),
                            Value::Dict(
                                [("key".into(), Value::Str("value".into()))]
                                    .into_iter()
                                    .collect(),
                            ),
                        ),
                    ]
                    .into_iter()
                    .collect()
                ),
                (
                    QUERY_KEY.into(),
                    [
                        ("query1".into(), Value::Str("/blog/".into())),
                        ("query2".into(), Value::Str("/projects/".into())),
                    ]
                    .into_iter()
                    .collect()
                )
            ]
            .into_iter()
            .collect()
        );
    }

    /// must be able to work even if the file is broken later on
    /// this is because we can't show fancy debug msgs here
    #[test]
    fn broken_file() {
        let src = r#"
        #metadata((
          title: "TITLE",
          my_bool: true,
          my_int: 10,
          my_array: (1, 2
          , 3, 4),
          my_dict: (
              key: "value"
          )
        )) <page>

        #metadata((query1: "/blog/")) <query>
        #metadata((query2: "/projects/")) <query>

        = My content!

        here is some *broken content
        
        "#;

        super::parse(src).unwrap();
    }
}
