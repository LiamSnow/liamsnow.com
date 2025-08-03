use std::sync::OnceLock;

use maud::{html, Markup};

use crate::template;

static HOME_HTML: OnceLock<Markup> = OnceLock::new();

pub fn init() {
    HOME_HTML.get_or_init(|| make_home_html());
}

pub async fn get_home() -> Markup {
    HOME_HTML.get().unwrap().clone()
}

pub fn make_home_html() -> Markup {
    template::apply(
        "/",
        "Home",
        html! {
            #name {
                // pre { "██╗     ██╗ █████╗ ███╗   ███╗    ███████╗███╗   ██╗ ██████╗ ██╗    ██╗" }
                // pre { "██║     ██║██╔══██╗████╗ ████║    ██╔════╝████╗  ██║██╔═══██╗██║    ██║" }
                // pre { "██║     ██║███████║██╔████╔██║    ███████╗██╔██╗ ██║██║   ██║██║ █╗ ██║" }
                // pre { "██║     ██║██╔══██║██║╚██╔╝██║    ╚════██║██║╚██╗██║██║   ██║██║███╗██║" }
                // pre { "███████╗██║██║  ██║██║ ╚═╝ ██║    ███████║██║ ╚████║╚██████╔╝╚███╔███╔╝" }
                // pre { "╚══════╝╚═╝╚═╝  ╚═╝╚═╝     ╚═╝    ╚══════╝╚═╝  ╚═══╝ ╚═════╝  ╚══╝╚══╝ " }
"
   ⣴⣶⣤⡤⠦⣤⣀⣤⠆     ⣈⣭⣿⣶⣿⣦⣼⣆
    ⠉⠻⢿⣿⠿⣿⣿⣶⣦⠤⠄⡠⢾⣿⣿⡿⠋⠉⠉⠻⣿⣿⡛⣦
          ⠈⢿⣿⣟⠦ ⣾⣿⣿⣷    ⠻⠿⢿⣿⣧⣄
           ⣸⣿⣿⢧ ⢻⠻⣿⣿⣷⣄⣀⠄⠢⣀⡀⠈⠙⠿⠄
          ⢠⣿⣿⣿⠈    ⣻⣿⣿⣿⣿⣿⣿⣿⣛⣳⣤⣀⣀
   ⢠⣧⣶⣥⡤⢄ ⣸⣿⣿⠘  ⢀⣴⣿⣿⡿⠛⣿⣿⣧⠈⢿⠿⠟⠛⠻⠿⠄
  ⣰⣿⣿⠛⠻⣿⣿⡦⢹⣿⣷   ⢊⣿⣿⡏  ⢸⣿⣿⡇ ⢀⣠⣄⣾⠄
 ⣠⣿⠿⠛ ⢀⣿⣿⣷⠘⢿⣿⣦⡀ ⢸⢿⣿⣿⣄ ⣸⣿⣿⡇⣪⣿⡿⠿⣿⣷⡄
 ⠙⠃   ⣼⣿⡟  ⠈⠻⣿⣿⣦⣌⡇⠻⣿⣿⣷⣿⣿⣿ ⣿⣿⡇ ⠛⠻⢷⣄
      ⢻⣿⣿⣄   ⠈⠻⣿⣿⣿⣷⣿⣿⣿⣿⣿⡟ ⠫⢿⣿⡆
       ⠻⣿⣿⣿⣿⣶⣶⣾⣿⣿⣿⣿⣿⣿⣿⣿⡟⢀⣀⣤⣾⡿⠃
"
            }
            p #caption { "This website was made using 🦀  and ❤️" }
            #links {
                a href="/about" {
                    p { "About" }
                    p { "a" }
                }
                a href="/projects" {
                    p { "Projects" }
                    p { "p" }
                }
                a href="/blog" {
                    p { "Blog" }
                    p { "b" }
                }
                a href="/links" {
                    p { "Links" }
                    p { "l" }
                }
            }
        },
        &[template::CSS_MAIN, template::CSS_HOME],
        &[template::JS_HOME]
    )
}

