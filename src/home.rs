use std::sync::OnceLock;

use maud::{Markup, html};

use crate::{
    post::{PostMeta, format_date},
    template,
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
                        a href="mailto:mail@liamsnow.com" { "EMAIL" }
                        a target="_blank" href="https://www.linkedin.com/in/william-snow-iv-140438169/" { "LINKEDIN" }
                        a target="_blank" href="https://github.com/liamsnow" { "GITHUB" }
                        a target="_blank" href="https://github.com/LiamSnow/resume/blob/main/resume.pdf" { "RESUME" }
                    }
                }
            }

            #sections {
                // section {
                //     .header {
                //         span { "/ SKILLS" }
                //         span { "■" }
                //     }
                //     .grid {
                //         div {
                //             h3 { "Systems Programming" }
                //             p {
                //                 "Networking, protocols, and concurrent systems. Check out "
                //                 a href="/projects/esphomebridge-rs" { "esphomebridge-rs" }
                //                 " & "
                //                 a href="/projects/opensleep" { "opensleep" }
                //             }
                //         }
                //         div {
                //             h3 { "Backend Development" }
                //             p {
                //                 "Fast APIs, SQL, NoSQL, embedded databases, REST, WebSockets, GraphQL"
                //             }
                //         }
                //     }
                // }

                section {
                    .header {
                        span { "/ PROJECTS" }
                        span { "■" }
                    }
                    .lined-grid {
                        @for (key, meta) in &recent_projects {
                           a href=(format!("/projects/{key}")) {
                               h3 { (meta.title) }
                               p.desc { (meta.desc) }
                               p.date { (format_date(&meta.date)) }
                           }
                        }
                    }
                }

                section {
                    .header {
                        span { "/ BLOG" }
                        span { "■" }
                    }
                    .lined-grid {
                        @for (key, meta) in &recent_blogs {
                           a href=(format!("/blog/{key}")) {
                               h3 { (meta.title) }
                               p.desc { (meta.desc) }
                               p.date { (format_date(&meta.date)) }
                           }
                        }
                    }
                }
            }
        },
        &[template::CSS_MAIN, template::CSS_HOME],
        &[],
        false,
    )
}
