use maud::{html, Markup};

use crate::page::make_page;

pub async fn get_home() -> Markup {
    make_page(
        "Home",
        html! {
            #mark-tl .mark { "+" }
            #mark-tr .mark { "+" }
            #mark-bl .mark { "+" }
            #mark-br .mark { "+" }

            main {
                #name {
                    "LIAM SNOW"
                    // pre { "██╗     ██╗ █████╗ ███╗   ███╗    ███████╗███╗   ██╗ ██████╗ ██╗    ██╗" }
                    // pre { "██║     ██║██╔══██╗████╗ ████║    ██╔════╝████╗  ██║██╔═══██╗██║    ██║" }
                    // pre { "██║     ██║███████║██╔████╔██║    ███████╗██╔██╗ ██║██║   ██║██║ █╗ ██║" }
                    // pre { "██║     ██║██╔══██║██║╚██╔╝██║    ╚════██║██║╚██╗██║██║   ██║██║███╗██║" }
                    // pre { "███████╗██║██║  ██║██║ ╚═╝ ██║    ███████║██║ ╚████║╚██████╔╝╚███╔███╔╝" }
                    // pre { "╚══════╝╚═╝╚═╝  ╚═╝╚═╝     ╚═╝    ╚══════╝╚═╝  ╚═══╝ ╚═════╝  ╚══╝╚══╝ " }
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
            }
        },
        html! {
            link rel="stylesheet" href="/static/home.css";
        },
    )
}
