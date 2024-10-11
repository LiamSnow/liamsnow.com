use std::time::Duration;

use leptos::*;

use crate::{
    components::display::Display,
    models::{grid::{Cell, Color, Grid, GridCoord}, menu::MenuLink},
};

#[component]
    pub fn MenuView(links: Vec<MenuLink>) -> impl IntoView {
    // let (grid, set_grid) = create_signal(Grid::new());
    let grid_size = create_rw_signal((0, 0));
    let half_menu_width = calc_half_menu_width(&links);

    let grid = create_memo(move |_| {

    });

    view! {
        <Display grid=grid grid_size=grid_size />
    }
}

fn set_cell(grid: &WriteSignal<Grid>, coord: GridCoord, cell: Cell) {
    grid.update(|g| {
        g.insert(coord, cell);
    });
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
