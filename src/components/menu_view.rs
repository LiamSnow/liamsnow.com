use std::rc::Rc;

use ev::{keydown, keypress};
use leptos::*;
use leptos_router::{NavigateOptions, State};
use leptos_use::{use_document, use_event_listener, use_window};
use log::info;

use crate::{
    components::display::Display,
    models::{
        grid::{push_block_to_grid, push_char_to_grid, push_str_to_grid, Cell, CellStyle, Color, Coord, TextAlign},
        menu::MenuLink,
    },
};

const TEXT_STYLE: CellStyle = CellStyle::basic(Color::WHITE, Color::NONE);
const BLOCK_STYLE: CellStyle = CellStyle::basic(Color::ORANGE, Color::NONE);
const SELECTED_STYLE: CellStyle = CellStyle::basic(Color::BLACK, Color::WHITE);
const SHORTCUT_STYLE: CellStyle = CellStyle::basic(Color::ORANGE, Color::NONE);
const CAPTION_STYLE: CellStyle = CellStyle::basic(Color::LIGHT_GREY, Color::NONE);
const HINT_STYLE: CellStyle = CellStyle::italic(Color::GREY, Color::NONE);

#[component]
pub fn MenuView(links: Vec<MenuLink>) -> impl IntoView {
    let num_links = links.len();
    let links = Rc::new(links);
    let grid_size = create_rw_signal((0, 0));
    let half_menu_width = calc_menu_width(&links) / 2;
    let (selected_link, set_selected_link) = create_signal(0);
    let name: Vec<&str> = vec![
        "██╗     ██╗ █████╗ ███╗   ███╗    ███████╗███╗   ██╗ ██████╗ ██╗    ██╗",
        "██║     ██║██╔══██╗████╗ ████║    ██╔════╝████╗  ██║██╔═══██╗██║    ██║",
        "██║     ██║███████║██╔████╔██║    ███████╗██╔██╗ ██║██║   ██║██║ █╗ ██║",
        "██║     ██║██╔══██║██║╚██╔╝██║    ╚════██║██║╚██╗██║██║   ██║██║███╗██║",
        "███████╗██║██║  ██║██║ ╚═╝ ██║    ███████║██║ ╚████║╚██████╔╝╚███╔███╔╝",
        "╚══════╝╚═╝╚═╝  ╚═╝╚═╝     ╚═╝    ╚══════╝╚═╝  ╚═══╝ ╚═════╝  ╚══╝╚══╝ ",
    ];
    let (links_offset, set_links_offset) = create_signal(0);

    let nav = |link: &str| {
        let is_external = link.contains(':');
        if is_external {
            let location = leptos_dom::helpers::location();
            let result = location.set_href(link).is_ok();
            info!("{}", result);
        }
        else {
            let navigate = leptos_router::use_navigate();
            navigate(link, Default::default());
        }
    };

    use_event_listener(use_document(), keydown, |event| {
        if event.ctrl_key() || !event.alt_key() && !event.meta_key() {

        }

        //10j, 10k
        //gg
        //G
        //10G or :10
    });

    use_hotkeys!(("keyj,arrowdown") => move |_| {
        set_selected_link((selected_link.get() + 1) % num_links);
    });

    use_hotkeys!(("keyk,arrowup") => move |_| {
        let cv = selected_link.get();
        set_selected_link(if cv == 0 { num_links-1 } else { cv - 1 });
    });

    let links_clone = links.clone();
    for link in links_clone.iter() {
        use_hotkeys!((format!("key{}", link.shortcut)) => move |_| {
            info!("na {}", link.link);
        });
    }

    let links_clone = links.clone();
    use_hotkeys!(("enter") => move |_| {
        let link = links_clone[selected_link.get()].link.as_str();
        nav(link);
    });

    let links_clone = links.clone();
    let grid = create_memo(move |_| {
        let gs = grid_size.get();
        let width = gs.0;
        let height = gs.1;

        let mut g: Vec<Cell> = vec![];

        if width < half_menu_width || height == 0 {
            return g;
        }

        let cx = width / 2;
        let cy = height / 2;

        let blocky = cy - 13;
        for (i, line) in name.iter().enumerate() {
            push_block_to_grid(&mut g, (cx, blocky+i), line, BLOCK_STYLE, TextAlign::CENTER);
        }

        push_str_to_grid(&mut g, (cx, cy-4), "This website was made using 🦀  and ❤️", CAPTION_STYLE, TextAlign::CENTER);
        // push_str_to_grid(&mut g, (cx, cy-3), "Use Vim keybinds (hjkl) or arrow keys to navigate.", HINT_STYLE, TextAlign::CENTER);

        let lx = cx - half_menu_width;
        let rx = cx + half_menu_width;
        let links_offset = 0;
        let selected_link = selected_link.get();

        for (i, link) in links_clone.iter().enumerate() {
            let y = cy + (i*2) + links_offset;
            let text_style = if selected_link == i { SELECTED_STYLE } else { TEXT_STYLE };
            push_str_to_grid(&mut g, (lx, y), &link.text, text_style, TextAlign::LEFT);
            push_char_to_grid(&mut g, (rx, y), link.shortcut, SHORTCUT_STYLE);
        }

        set_links_offset(links_offset);

        g
    });

    let links_clone = links.clone();
    let on_click = move |coord: Coord| {
        let y = coord.1;
        let cy = grid_size.get().1 / 2;
        let offset = links_offset.get();
        let min_y = cy + offset;
        let max_y = min_y + (num_links * 2);

        if y < min_y || y > max_y {
            return;
        }

        let new_index = (y - min_y) / 2;
        if new_index == selected_link.get() {
            let link = links_clone[new_index].link.as_str();
            nav(link);
        }
        else {
            set_selected_link(new_index);
        }
    };

    view! {
        <Display
            grid=grid
            grid_size=grid_size
            on_click=on_click
        />
    }
}

fn calc_menu_width(links: &Vec<MenuLink>) -> usize {
    let mut longest_text = 0;
    for link in links.iter() {
        let text_len = link.text.len();
        if text_len > longest_text {
            longest_text = text_len
        }
    }

    longest_text + 8 + 1
}
