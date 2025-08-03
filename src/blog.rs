use std::collections::HashMap;
use std::sync::OnceLock;

use axum::extract::Path;
use maud::{html, Markup};

use crate::markdown::{process_dir, BlogPage};
use crate::page::make_page;

static BLOG_MAP: OnceLock<HashMap<String, BlogPage>> = OnceLock::new();
static HOME_HTML: OnceLock<Markup> = OnceLock::new();

pub fn init() {
    BLOG_MAP.get_or_init(|| process_dir("./blog"));
    HOME_HTML.get_or_init(|| make_home_html());
}

pub fn make_home_html() -> Markup {
    let map = BLOG_MAP.get().unwrap();

    // let mut h = html! {};
    // for (key, value) in &*map {
    //     h = html! {
    //       (h)
    //       a href="" {
    //         h3 { value.meta.title }
    //         p { value.meta.desc }
    //         p { value.meta.date }
    //       }
    //     };
    // }

          // @for (key, value) in &*map {
          //    a href=(key) {
          //        p { value.meta.desc }
          //        h3 { value.meta.title }
          //        p { value.meta.date }
          //    }
          // }

    make_page(
        "/blog",
        "Blog",
        html! {
          @for (key, value) in &*map {
             a href=(format!("/blog/{key}")) {
                 p { (value.meta.desc) }
                 h3 { (value.meta.title) }
                 p { (value.meta.date) }
             }
          }

        },
        html! {},
    )
}

pub async fn get_home() -> Markup {
    HOME_HTML.get().unwrap().clone()
}

pub async fn get_blog(Path(params): Path<HashMap<String, String>>) -> Markup {
    let idp = params.get("id");
    if idp.is_none() {
        return html! { "Must provide blog!" };
    }
    let id = idp.unwrap();

    let bpo = BLOG_MAP.get().unwrap().get(id);
    if bpo.is_none() {
        return html! { "Blog does not exist!" };
    }
    let bp = bpo.unwrap();

    make_page(
        format!("/blog/{id}").as_str(),
        "Blog",
        html! {
            (bp.html)
        },
        html! {
            link rel="stylesheet" href="/static/markdown.css";
        },
    )
}
