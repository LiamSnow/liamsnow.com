use leptos::*;
use leptos_hotkeys::use_hotkeys;
use log::info;

use crate::{
    components::display::Display,
    models::{
        grid::{push_char_to_grid, push_str_to_grid, Cell, CellStyle, Color, TextAlign},
        menu::MenuLink,
    },
};

const TEXT_STYLE: CellStyle = CellStyle {
    foreground: Color::WHITE,
    background: Color::NONE,
    bold: false,
    italic: false,
};
const SELECTED_STYLE: CellStyle = CellStyle {
    foreground: Color::BLACK,
    background: Color::WHITE,
    bold: false,
    italic: false,
};
const SHORTCUT_STYLE: CellStyle = CellStyle {
    foreground: Color::ORANGE,
    background: Color::NONE,
    bold: false,
    italic: false,
};

#[component]
pub fn MenuView(links: Vec<MenuLink>) -> impl IntoView {
    let grid_size = create_rw_signal((0, 0));
    let half_menu_width = calc_half_menu_width(&links);
    let num_links = links.len();
    let (selected_link, set_selected_link) = create_signal(0);

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
        let lx = cx - half_menu_width;
        let rx = cx + half_menu_width;

        for (i, link) in links.iter().enumerate() {
            let y = cy + i;
            let text_style = if selected_link.get() == i { SELECTED_STYLE } else { TEXT_STYLE };
            push_str_to_grid(&mut g, (lx, y), &link.text, text_style, TextAlign::LEFT);
            push_char_to_grid(&mut g, (rx, y), link.shortcut, SHORTCUT_STYLE);
        }

        g
    });

    view! {
        <Display grid=grid grid_size=grid_size />
    }
}

fn calc_half_menu_width(links: &Vec<MenuLink>) -> usize {
    let mut longest_text = 0;
    for link in links.iter() {
        let text_len = link.text.len();
        if text_len > longest_text {
            longest_text = text_len
        }
    }

    (longest_text + 2 + 1) / 2
}
