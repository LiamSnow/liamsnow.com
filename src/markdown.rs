use std::{collections::HashMap, fs, io::BufWriter, path::PathBuf};

use comrak::{format_html_with_plugins, markdown_to_html_with_plugins, parse_document, plugins::syntect::SyntectAdapter, Arena, Options, Plugins};
use maud::{Markup, PreEscaped};

pub struct BlogPage {
    content: Markup,
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
            let arena = Arena::new();
            let root = parse_document(&arena, content.as_str(), options);
            let mut bw = BufWriter::new(Vec::new());
            format_html_with_plugins(root, options, &mut bw, plugins).unwrap();
            let html = String::from_utf8(bw.into_inner().unwrap()).unwrap();

            Some(BlogPage {
                content: PreEscaped(html)
            })
        },
    }
}

// content: PreEscaped(markdown_to_html_with_plugins(c.as_str(), &options, &plugins)),
