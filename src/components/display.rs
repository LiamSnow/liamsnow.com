use crate::models::display::{Cell, GridInfo, CELL_HEIGHT, CELL_WIDTH};
use leptos::html::Div;
use leptos::*;
use leptos_use::use_resize_observer;
use log::info;

#[component]
pub fn Display() -> impl IntoView {
    let mut grid: Vec<Vec<Cell>>;

    let display = create_node_ref::<Div>();
    let (text, set_text) = create_signal("".to_string());

    use_resize_observer(display, move |entries, observer| {
        let rect = entries[0].content_rect();
        set_text.set(format!(
            "width: {}\nheight: {}",
            rect.width(),
            rect.height()
        ));
        info!("12");
    });

    view! {
        <div class="display" node_ref=display>
        { move || text.get() }
        </div>
    }
}

fn calc_grid_info(screen_width: u32, screen_height: u32) -> GridInfo {
    return GridInfo {
        x_cells: screen_width / CELL_WIDTH,
        y_cells: screen_height / CELL_HEIGHT,
        x_padding: screen_width % CELL_WIDTH,
        y_padding: screen_height % CELL_HEIGHT,
    };
}
