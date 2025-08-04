use jiff::civil::DateTime;
use maud::{Markup, html};
use std::{collections::HashMap, fs, path::PathBuf};

use crate::{
    markdown::{self, KATEX_CSS, KATEX_CSS_HASH, KATEX_JS, KATEX_JS_HASH, get_katex_run_js},
    template,
};

pub struct PostCollection {
    pub posts: HashMap<String, Post>,
    pub index: Markup,
}

pub struct Post {
    pub html: Markup,
    pub meta: PostMeta,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct PostMeta {
    pub title: String,
    pub desc: String,
    pub date: DateTime,
    #[serde(default)]
    pub homepage: bool,
}

/// This collection is used for both blogs and projects
/// On creation, it will read all markdown files in the
/// respective folder, process them, and pre-generate
/// all the pages (index + individual pages)
impl PostCollection {
    /// returns a new PostCollection + posts marked for homepage
    pub fn new(collection: String) -> (Self, Vec<(String, PostMeta)>) {
        let collection_lower = collection.to_lowercase();
        let posts = Self::process_dir(&collection);

        let mut posts_sorted: Vec<(String, PostMeta)> = posts
            .iter()
            .map(|(key, post)| (key.clone(), post.meta.clone()))
            .collect();
        posts_sorted.sort_by(|a, b| b.1.date.cmp(&a.1.date));

        let index = template::apply(
            &format!("/{collection_lower}"),
            &format!("Liam Snow's {collection}"),
            &format!("Liam Snow's {collection}. Programming, systems, backend, Rust and more."),
            "index",
            html! {},
            html! {
              @for (key, meta) in &posts_sorted {
                 a .post href=(format!("/{collection_lower}/{key}")) {
                     h2.title { (meta.title) }
                     p.desc { (meta.desc) }
                     p.date { (fancy_date_format(&meta.date)) }
                 }
              }

            },
            None,
        );

        let homepage_posts: Vec<(String, PostMeta)> = posts_sorted
            .into_iter()
            .filter(|(_, meta)| meta.homepage)
            .collect();

        (PostCollection { posts, index }, homepage_posts)
    }

    pub fn get_post(&self, params: HashMap<String, String>) -> Markup {
        let id_option = params.get("id");
        if id_option.is_none() {
            return html! { "Must provide post ID!" };
        }
        let id = id_option.unwrap();

        let post_option = self.posts.get(id);
        if post_option.is_none() {
            return html! { "Post does not exist!" };
        }
        let post = post_option.unwrap();

        post.html.clone()
    }

    fn process_dir(collection: &str) -> HashMap<String, Post> {
        let mut map = HashMap::new();
        let paths = fs::read_dir(format!("./{}", collection.to_lowercase())).unwrap();
        for pathres in paths {
            let path = pathres.unwrap().path();
            let filename = path.file_stem().unwrap().to_str().unwrap().to_string();
            let post = Post::from_file(collection, &filename, &path);
            map.insert(filename, post);
        }
        map
    }
}

impl Post {
    fn from_file(collection: &str, filename: &str, path: &PathBuf) -> Post {
        let md = fs::read_to_string(path).expect("Could not read markdown file");
        let meta = PostMeta::from_markdown(&md);
        let content = markdown::to_html(&md);
        let needs_katex = content.0.contains("data-math-style");

        let schema_type = if collection.to_lowercase() == "blog" {
            "Article"
        } else {
            "CreativeWork"
        };

        let structured_data = format!(
            r#"{{
                "@context": "https://schema.org",
                "@type": "{}",
                "headline": "{}",
                "description": "{}",
                "datePublished": "{}",
                "dateModified": "{}",
                "author": {{
                    "@type": "Person",
                    "name": "William Snow IV",
                    "url": "https://liamsnow.com"
                }},
                "publisher": {{
                    "@type": "Person",
                    "name": "William Snow IV",
                    "url": "https://liamsnow.com"
                }},
                "url": "https://liamsnow.com/{}/{}"
            }}"#,
            schema_type,
            meta.title.replace('"', r#"\""#),
            meta.desc.replace('"', r#"\""#),
            meta.date.strftime("%Y-%m-%d"),
            meta.date.strftime("%Y-%m-%d"),
            collection.to_lowercase(),
            filename
        );

        let html = template::apply(
            &format!("/{}/{}", collection.to_lowercase(), filename),
            &meta.title,
            &meta.desc,
            "post",
            html! {
                @if needs_katex {
                    link rel="preconnect" href="https://cdn.jsdelivr.net" crossorigin="anonymous";
                    link rel="preload" as="style" href=(KATEX_CSS) integrity=(KATEX_CSS_HASH) crossorigin="anonymous";
                    link rel="stylesheet" href=(KATEX_CSS) integrity=(KATEX_CSS_HASH) crossorigin="anonymous" media="print" onload="this.media='all'";
                    script defer src=(KATEX_JS) integrity=(KATEX_JS_HASH) crossorigin="anonymous" {}
                }
            },
            html! {
                a.post-back href=(format!("/{}", collection.to_lowercase())) { "← Back" }
                h1.post-title { (meta.title) }
                p.post-date { (fancy_date_format(&meta.date)) }

                (content)

                @if needs_katex {
                    script {
                        (get_katex_run_js())
                    }
                }
            },
            Some(&structured_data),
        );

        Post { meta, html }
    }
}

impl PostMeta {
    fn from_markdown(content: &str) -> Self {
        let yaml_str = content
            .split("---")
            .nth(1)
            .map(|s| s.trim())
            .expect("Couldn't find frontmatter delimeters");
        serde_yaml::from_str(yaml_str).expect("Serde YAML failed to parse frontmatter")
    }
}

/// returns: Aug 3st 2025
pub fn fancy_date_format(dt: &DateTime) -> String {
    let day = dt.day();

    let suffix = match day {
        1 | 21 | 31 => "st",
        2 | 22 => "nd",
        3 | 23 => "rd",
        _ => "th",
    };

    format!("{} {}{} {}", dt.strftime("%b"), day, suffix, dt.year())
}
