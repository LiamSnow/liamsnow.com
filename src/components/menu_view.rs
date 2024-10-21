use std::time::Duration;

use leptos::*;
use leptos_use::{use_interval, UseIntervalReturn};
use log::info;

use crate::{
    components::display::Display,
    models::{
        grid::{insert_string, Cell, CellStyle, Color, Grid, GridCoord, TextAlign},
        menu::MenuLink,
    },
};

#[component]
pub fn MenuView(links: Vec<MenuLink>) -> impl IntoView {
    let grid_size = create_rw_signal((0, 0));
    let half_menu_width = calc_half_menu_width(&links);
    let UseIntervalReturn { counter, .. } = use_interval(500);

    let grid = create_memo(move |_| {
        let gs = grid_size.get();
        let mx = gs.0;
        let my = gs.1;

        let v = counter.get() % 5;

        let mut g = Grid::new();


        let style = CellStyle {
            background: Color::RED,
            foreground: Color::BLUE,
            bold: false,
            italic: false,
        };

        insert_string(&mut g, (mx/2, my/2), "Hello", style, TextAlign::RIGHT);
        g.insert((mx / 2, my / 2), Cell { char: 'x', style });

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
