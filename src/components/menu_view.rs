use std::time::Duration;

use leptos::*;
use log::info;

use crate::{
    components::display::Display,
    models::{grid::{Cell, Color, Grid, GridCoord}, menu::MenuLink},
};

#[component]
pub fn MenuView(links: Vec<MenuLink>) -> impl IntoView {
    let grid_size = create_rw_signal((0, 0));
    let half_menu_width = calc_half_menu_width(&links);


    let grid = create_memo(move |_| {
        let gs = grid_size.get();
        let mx = gs.0;
        let my = gs.1;
        let mut g = Grid::new();

        g.insert((0, 0), Cell {
            char: 'a',
            foreground: Color::WHITE,
            background: Color::NONE,
            italic: false,
            bold: false
        });

        g.insert((mx/2, my/2), Cell {
            char: 'a',
            foreground: Color::WHITE,
            background: Color::NONE,
            italic: false,
            bold: false
        });

        g.insert((mx-1, my-1), Cell {
            char: 'a',
            foreground: Color::WHITE,
            background: Color::NONE,
            italic: false,
            bold: false
        });

        g.insert((mx-1, 0), Cell {
            char: 'a',
            foreground: Color::WHITE,
            background: Color::NONE,
            italic: false,
            bold: false
        });

        g.insert((0, my-1), Cell {
            char: 'a',
            foreground: Color::WHITE,
            background: Color::NONE,
            italic: false,
            bold: false
        });

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
