use std::sync::OnceLock;

use comrak::{
    ExtensionOptions, Options, ParseOptions, Plugins, RenderOptions, markdown_to_html_with_plugins,
    plugins::syntect::SyntectAdapter,
};
use maud::PreEscaped;

use crate::template::load_js;

pub const KATEX_CSS: &str = "https://cdn.jsdelivr.net/npm/katex@0.16.22/dist/katex.min.css";
pub const KATEX_CSS_HASH: &str =
    "sha384-5TcZemv2l/9On385z///+d7MSYlvIEw9FuZTIdZ14vJLqWphw7e7ZPuOiCHJcFCP";
pub const KATEX_JS: &str = "https://cdn.jsdelivr.net/npm/katex@0.16.22/dist/katex.min.js";
pub const KATEX_JS_HASH: &str =
    "sha384-cMkvdD8LoxVzGF/RPUKAcvmm49FQ0oxwDF3BGKtDXcEc+T1b2N+teh/OJfpU0jr6";
static KATEX_RUN_JS: OnceLock<PreEscaped<String>> = OnceLock::new();

static PLUGINS: OnceLock<Plugins> = OnceLock::new();
static OPTIONS: OnceLock<Options> = OnceLock::new();
static SYNTECT: OnceLock<SyntectAdapter> = OnceLock::new();

pub fn get_syntect_adapter() -> &'static SyntectAdapter {
    SYNTECT.get_or_init(|| SyntectAdapter::new(Some("base16-ocean.light")))
}

pub fn make_plugins() -> Plugins<'static> {
    let mut plugs = Plugins::default();
    plugs.render.codefence_syntax_highlighter = Some(get_syntect_adapter());
    plugs
}

pub fn make_options() -> Options<'static> {
    Options {
        extension: ExtensionOptions {
            strikethrough: true,
            tagfilter: false,
            table: false,
            autolink: false,
            tasklist: true,
            superscript: true,
            header_ids: Some("".to_string()),
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
    }
}

pub fn to_html(content: &str) -> PreEscaped<String> {
    PreEscaped(markdown_to_html_with_plugins(
        content,
        OPTIONS.get_or_init(make_options),
        PLUGINS.get_or_init(make_plugins),
    ))
}

pub fn get_katex_run_js(base_dir: &str) -> &'static PreEscaped<String> {
    KATEX_RUN_JS.get_or_init(|| load_js(base_dir, "katex_run"))
}
