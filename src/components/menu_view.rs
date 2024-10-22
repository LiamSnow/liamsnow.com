use leptos::*;
use leptos_hotkeys::use_hotkeys;
use leptos_use::{use_interval, UseIntervalReturn};
use log::info;

use crate::{
    components::display::Display,
    models::{
        grid::{push_block_to_grid, push_char_to_grid, push_str_to_grid, Cell, CellStyle, Color, TextAlign},
        menu::MenuLink,
    },
};

const TEXT_STYLE: CellStyle = CellStyle::basic(Color::WHITE, Color::NONE);
const BLOCK_STYLE: CellStyle = CellStyle::basic(Color::ORANGE, Color::NONE);
const SELECTED_STYLE: CellStyle = CellStyle::basic(Color::BLACK, Color::WHITE);
const SHORTCUT_STYLE: CellStyle = CellStyle::basic(Color::ORANGE, Color::NONE);

#[component]
pub fn MenuView(links: Vec<MenuLink>) -> impl IntoView {
    let grid_size = create_rw_signal((0, 0));
    let half_menu_width = calc_menu_width(&links) / 2;
    let num_links = links.len();
    let (selected_link, set_selected_link) = create_signal(0);
    let name: Vec<&str> = vec![
        "██╗     ██╗ █████╗ ███╗   ███╗    ███████╗███╗   ██╗ ██████╗ ██╗    ██╗",
        "██║     ██║██╔══██╗████╗ ████║    ██╔════╝████╗  ██║██╔═══██╗██║    ██║",
        "██║     ██║███████║██╔████╔██║    ███████╗██╔██╗ ██║██║   ██║██║ █╗ ██║",
        "██║     ██║██╔══██║██║╚██╔╝██║    ╚════██║██║╚██╗██║██║   ██║██║███╗██║",
        "███████╗██║██║  ██║██║ ╚═╝ ██║    ███████║██║ ╚████║╚██████╔╝╚███╔███╔╝",
        "╚══════╝╚═╝╚═╝  ╚═╝╚═╝     ╚═╝    ╚══════╝╚═╝  ╚═══╝ ╚═════╝  ╚══╝╚══╝ ",
    ];
    let UseIntervalReturn { counter, .. } = use_interval(400);

    use_hotkeys!(("keyj") => move |_| {
        set_selected_link((selected_link.get() + 1) % num_links);
    });

    use_hotkeys!(("keyk") => move |_| {
        let cv = selected_link.get();
        set_selected_link(if cv == 0 { num_links-1 } else { cv - 1 });
    });

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

        for (i, line) in name.iter().enumerate() {
            push_block_to_grid(&mut g, (cx, cy-13+i), line, BLOCK_STYLE, TextAlign::CENTER);
        }

        push_str_to_grid(&mut g, (cx, cy-4), "This website was made using 🦀  and ❤️", TEXT_STYLE, TextAlign::CENTER);

        let lx = cx - half_menu_width;
        let rx = cx + half_menu_width;
        for (i, link) in links.iter().enumerate() {
            let y = cy + (i*2);
            let active = selected_link.get() == i && counter.get() % 3 != 0;
            let text_style = if active { SELECTED_STYLE } else { TEXT_STYLE };
            push_str_to_grid(&mut g, (lx, y), &link.text, text_style, TextAlign::LEFT);
            push_char_to_grid(&mut g, (rx, y), link.shortcut, SHORTCUT_STYLE);
        }

        g
    });

    view! {
        <Display grid=grid grid_size=grid_size />
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
