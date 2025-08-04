use std::{collections::HashMap, fs, path::PathBuf};

use comrak::{
    ExtensionOptions, Options, ParseOptions, Plugins, RenderOptions, markdown_to_html_with_plugins,
    plugins::syntect::SyntectAdapter,
};
use jiff::civil::DateTime;
use maud::{Markup, PreEscaped, html};

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
    pub date: DateTime,
}

impl PostCollection {
    /// returns a new PostCollection + 3 most recent posts
    pub fn new(col_name: String) -> (Self, Vec<(String, PostMeta)>) {
        let col_name_lower = col_name.to_lowercase();
        let posts = process_dir(&col_name);

        let mut posts_sorted: Vec<(String, PostMeta)> = posts
            .iter()
            .map(|(key, post)| (key.clone(), post.meta.clone()))
            .collect();
        posts_sorted.sort_by(|a, b| b.1.date.cmp(&a.1.date));

        let index = template::apply(
            &format!("/{col_name_lower}"),
            &col_name,
            html! {
              @for (key, meta) in &posts_sorted {
                 a .post href=(format!("/{col_name_lower}/{key}")) {
                     h2.title { (meta.title) }
                     p.desc { (meta.desc) }
                     p.date { (format_date(&meta.date)) }
                 }
              }

            },
            &[template::CSS_MAIN, template::CSS_INDEX],
            &[],
            false,
        );

        let recent_posts = posts_sorted.iter().take(3).cloned().collect();

        (PostCollection { posts, index }, recent_posts)
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
    // print available themes
    // let ts = ThemeSet::load_defaults();
    // for key in ts.themes.keys() {
    //     println!("{key}");
    // }

    let mut plugs = Plugins::default();
    let adap = SyntectAdapter::new(Some("base16-ocean.light"));
    plugs.render.codefence_syntax_highlighter = Some(&adap);

    let opts = Options {
        extension: ExtensionOptions {
            strikethrough: true,
            tagfilter: false,
            table: false,
            autolink: false,
            tasklist: true,
            superscript: true,
            header_ids: None,
            footnotes: true,
            description_lists: true,
            front_matter_delimiter: Some("---".to_owned()),
            multiline_block_quotes: true,
            alerts: true,
            math_dollars: true,
            math_code: false,
            wikilinks_title_after_pipe: false,
            wikilinks_title_before_pipe: false,
            underline: true,
            subscript: true,
            spoiler: true,
            greentext: true,
            image_url_rewriter: None,
            link_url_rewriter: None,
        },
        parse: ParseOptions {
            smart: true,
            default_info_string: None,
            relaxed_tasklist_matching: true,
            relaxed_autolinks: false,
            broken_link_callback: None,
        },
        render: RenderOptions::default(),
    };

    let mut map = HashMap::new();
    let paths = fs::read_dir(format!("./{}", collection_name.to_lowercase())).unwrap();
    for pathres in paths {
        let path = pathres.unwrap().path();
        let filename = path.file_stem().unwrap().to_str().unwrap().to_string();
        let post = process_file(collection_name, &filename, &path, &opts, &plugs);
        map.insert(filename, post);
    }
    map
}

fn process_file(
    collecion_name: &str,
    filename: &str,
    path: &PathBuf,
    options: &Options,
    plugins: &Plugins,
) -> Post {
    let content = fs::read_to_string(path).expect("Could not read markdown file");
    let meta = process_front_matter(&content);

    let markdown = PreEscaped(markdown_to_html_with_plugins(&content, options, plugins));

    let html = template::apply(
        &format!("/{}/{}", collecion_name.to_lowercase(), filename),
        &meta.title,
        html! {
            a.post-back href=(format!("/{}", collecion_name.to_lowercase())) { "← Back" }
            h1.post-title { (meta.title) }
            p.post-date { (format_date(&meta.date)) }
            (markdown)
        },
        &[template::CSS_MAIN, template::CSS_POST],
        &[template::KATEX],
        true,
    );

    Post { meta, html }
}

fn process_front_matter(content: &str) -> PostMeta {
    let yaml_str = content
        .split("---")
        .nth(1)
        .map(|s| s.trim())
        .expect("Couldn't find frontmatter delimeters");
    serde_yaml::from_str(yaml_str).expect("Serde YAML failed to parse frontmatter")
}

pub fn format_date(dt: &DateTime) -> String {
    let day = dt.day();

    let suffix = match day {
        1 | 21 | 31 => "st",
        2 | 22 => "nd",
        3 | 23 => "rd",
        _ => "th",
    };

    format!("{} {}{} {}", dt.strftime("%b"), day, suffix, dt.year())
}
