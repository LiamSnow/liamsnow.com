use crate::{components::menu_view::MenuView, models::menu::MenuLink};
use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    let links = vec![
        MenuLink {
            text: "About".to_string(),
            link: "/about".to_string(),
            shortcut: 'a'
        },
        MenuLink {
            text: "Projects".to_string(),
            link: "/projects".to_string(),
            shortcut: 'p'
        },
        MenuLink {
            text: "Blog".to_string(),
            link: "/blog".to_string(),
            shortcut: 'b'
        },
        MenuLink {
            text: "Github".to_string(),
            link: "https://github.com/liamsnow".to_string(),
            shortcut: 'g'
        },
        MenuLink {
            text: "LinkedIn".to_string(),
            link: "https://www.linkedin.com/in/william-snow-iv-140438169/".to_string(),
            shortcut: 'l'
        },
        MenuLink {
            text: "Email".to_string(),
            link: "mailto:mail@liamsnow.com".to_string(),
            shortcut: 'e'
        },
        MenuLink {
            text: "Resume".to_string(),
            link: "/resume".to_string(),
            shortcut: 'r'
        },
    ];

    view! {
        <MenuView links=links/>
    }
}
