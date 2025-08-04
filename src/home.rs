use std::sync::OnceLock;

use maud::{Markup, html};

use crate::{
    post::{PostMeta, format_date},
    template::{self, link_new_tab},
};

static HOME_HTML: OnceLock<Markup> = OnceLock::new();

pub fn init(recent_projects: Vec<(String, PostMeta)>, recent_blogs: Vec<(String, PostMeta)>) {
    HOME_HTML.get_or_init(|| make_home_html(recent_projects, recent_blogs));
}

pub async fn get_home() -> Markup {
    HOME_HTML.get().unwrap().clone()
}

const ABOUT_ME: &str = "
I'm a Computer Science MS and Electrical & Computer Engineering BS student at
Worcester Polytechnic Institute with a passion for systems programming and backend
development.
I enjoy learning in all areas of CS and have experience in many languages:
Rust, Golang, C, C++, Python, Java, C#, TypeScript, JavaScript, Lua, Swift, and others.
";

pub fn make_home_html(
    recent_projects: Vec<(String, PostMeta)>,
    recent_blogs: Vec<(String, PostMeta)>,
) -> Markup {
    template::apply(
        "/",
        "Home",
        html! {
            #hero {
                h1.desktop {
                    "LIAMSNOW"
                }

                h1.mobile{
                    "LIAM"
                    br;
                    "SNOW"
                }

                .content {
                    .left {}
                    .right {
                        h2 { "About me" }
                        p { (ABOUT_ME) }
                        (link_new_tab("EMAIL", "mailto:mail@liamsnow.com"))
                        (link_new_tab("LINKEDIN", "https://www.linkedin.com/in/william-snow-iv-140438169/"))
                        (link_new_tab("GITHUB", "https://github.com/liamsnow"))
                        (link_new_tab("RESUME", "https://github.com/LiamSnow/resume/blob/main/resume.pdf"))
                    }
                }
            }

            #sections {
                (make_section("PROJECTS", &recent_projects))
                (make_section("BLOG", &recent_blogs))
            }
        },
        html! {},
        "home",
    )
}

fn make_section(name: &str, items: &Vec<(String, PostMeta)>) -> Markup {
    html! {
        section {
            .header {
                span { "/ " (name) }
                span { "■" }
            }
            .grid {
                @for (key, meta) in items {
                   a href=(format!("/{}/{key}", name.to_lowercase())) {
                       h3 { (meta.title) }
                       p.desc { (meta.desc) }
                       p.date { (format_date(&meta.date)) }
                   }
                }
            }
        }
    }
}
