use std::{collections::HashMap, fs, path::PathBuf};

use comrak::{markdown_to_html_with_plugins, plugins::syntect::SyntectAdapter, Options, Plugins};
use maud::{html, Markup, PreEscaped};

use serde_yaml;

use crate::template;

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
    pub date: String,
}

impl PostCollection {
    pub fn new(name: String) -> Self {
        let name_lower = name.to_lowercase();
        let posts = process_dir(&name);

        let index = template::apply(
            &format!("/{name_lower}"),
            &name,
            html! {
              @for (key, value) in &posts {
                 a .post href=(format!("/{name_lower}/{key}")) {
                     h2.title { (value.meta.title) }
                     p.desc { (value.meta.desc) }
                     p.date { (value.meta.date) }
                 }
              }

            },
            &[template::CSS_MAIN, template::CSS_INDEX],
            &[]
        );

        PostCollection { posts, index }
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
}

fn process_dir(collection_name: &str) -> HashMap<String, Post> {
    let mut plugs = Plugins::default();
    let adap = SyntectAdapter::new(Some("base16-eighties.dark"));
    plugs.render.codefence_syntax_highlighter = Some(&adap);

    let mut opts = Options::default();
    opts.extension.strikethrough = true;
    opts.extension.table = true;
    opts.extension.superscript = true;
    opts.extension.front_matter_delimiter = Some("---".to_owned());
    opts.extension.math_dollars = true;

    let mut map = HashMap::new();
    let paths = fs::read_dir(&format!("./{}", collection_name.to_lowercase())).unwrap();
    for pathres in paths {
        let path = pathres.unwrap().path();
        let filename = path.file_stem().unwrap().to_str().unwrap().to_string();
        let post = process_file(collection_name, &filename, &path, &opts, &plugs);
        if post.is_some() {
            map.insert(filename, post.unwrap());
        }
    }
    map
}

fn process_file(
    collecion_name: &str,
    filename: &str,
    path: &PathBuf,
    options: &Options,
    plugins: &Plugins,
) -> Option<Post> {
    let content_opt = fs::read_to_string(path);
    match content_opt {
        Err(_) => None,
        Ok(content) => {
            let meta_option = process_front_matter(&content);
            if meta_option.is_none() {
                return None;
            }
            let meta = meta_option.unwrap();

            let markdown = PreEscaped(markdown_to_html_with_plugins(&content, &options, &plugins));

            let html = template::apply(
                &format!("/{}/{}", collecion_name.to_lowercase(), filename),
                &meta.title,
                html! {
                    (markdown)
                },
                &[template::CSS_MAIN, template::CSS_POST],
                &[]
            );

            Some(Post { meta, html })
        }
    }
}

fn process_front_matter(content: &String) -> Option<PostMeta> {
    let yaml_str = content.split("---").nth(1).map(|s| s.trim());
    if yaml_str.is_none() {
        return None;
    }
    serde_yaml::from_str(yaml_str.unwrap()).ok()
}
