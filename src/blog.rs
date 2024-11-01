use std::collections::HashMap;
use std::sync::OnceLock;

use axum::extract::Path;
use maud::{html, Markup};

use crate::markdown::process_dir;
use crate::page::make_page;

static BLOG_MAP: OnceLock<HashMap<String, Markup>> = OnceLock::new();
pub fn init() {
    BLOG_MAP.get_or_init(|| process_dir("./blog"));
}

pub async fn get_home() -> Markup {
    let map = BLOG_MAP.get().unwrap();
    let keys = map.keys()

    make_page(
        "/blog",
        "Blog",
        html! { "Blogs" },
        html! { },
    )
}

pub async fn get_blog(Path(params): Path<HashMap<String, String>>) -> Markup {
    let idp = params.get("id");
    if idp.is_none() {
        return html! { "Must provide blog!" };
    }
    let id = idp.unwrap();

    let mdp = BLOG_MAP.get().unwrap().get(id);
    if mdp.is_none() {
      return html! { "Blog does not exist!" }
    }
    let md = mdp.unwrap();

    make_page(
        format!("/blog/{id}").as_str(),
        "Blog",
        html! {
          (md)
        },
        html! {
            link rel="stylesheet" href="/static/markdown.css";
        },
    )
}
