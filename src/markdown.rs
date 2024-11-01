use std::{collections::HashMap, fs, path::PathBuf};

use comrak::{markdown_to_html_with_plugins, plugins::syntect::SyntectAdapter, Options, Plugins};
use maud::{Markup, PreEscaped};

use serde_yaml;

pub struct BlogPage {
    pub html: Markup,
    pub meta: BlogPageMeta,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct BlogPageMeta {
    pub title: String,
    pub desc: String,
    pub date: String,
}

pub fn process_dir(path: &str) -> HashMap<String, BlogPage> {
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
    let paths = fs::read_dir(path).unwrap();
    for pathres in paths {
        let path = pathres.unwrap().path();
        let filename = path.file_stem().unwrap().to_str().unwrap().to_string();
        let blogpage = process_file(&path, &opts, &plugs);
        if blogpage.is_some() {
            map.insert(filename, blogpage.unwrap());
        }
    }
    map
}

fn process_file(path: &PathBuf, options: &Options, plugins: &Plugins) -> Option<BlogPage> {
    let content_opt = fs::read_to_string(path);
    match content_opt {
        Err(_) => None,
        Ok(content) => {
            let html = PreEscaped(markdown_to_html_with_plugins(
                content.as_str(),
                &options,
                &plugins,
            ));
            let meta = process_front_matter(&content);
            if meta.is_none() {
                return None;
            }

            Some(BlogPage {
                meta: meta.unwrap(),
                html,
            })
        }
    }
}

fn process_front_matter(content: &String) -> Option<BlogPageMeta> {
    let yaml_str = content.split("---").nth(1).map(|s| s.trim());
    if yaml_str.is_none() {
        return None;
    }
    serde_yaml::from_str(yaml_str.unwrap()).ok()
}
