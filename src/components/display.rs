use crate::models::grid::{Cell, Grid, GridCoord, CELL_WIDTH, LINE_HEIGHT};
use leptos::html::Div;
use leptos::*;
use leptos_use::{use_debounce_fn_with_options, use_element_size, use_resize_observer, DebounceOptions, UseElementSizeReturn};

#[component]
pub fn Display(grid: Memo<Grid>, grid_size: RwSignal<GridCoord>) -> impl IntoView {
    let (padding, set_padding) = create_signal((0.0, 0.0));
    let wrapper = NodeRef::<Div>::new();
    let UseElementSizeReturn { width, height } = use_element_size(wrapper);

    let handle_resize = move || {
        let w = width.get();
        let h = height.get();
        let x = w / CELL_WIDTH;
        let y = h / LINE_HEIGHT;
        let x_cells = x.floor() as usize;
        let y_cells = y.floor() as usize;
        grid_size.update(|size| *size = (x_cells, y_cells));
        let x_padding = (w - (x_cells as f64 * CELL_WIDTH)) / 2.0;
        let y_padding = (h - (y_cells as f64 * LINE_HEIGHT)) / 2.0;
        set_padding((x_padding, y_padding));
    };
    let handle_resize_debounced = use_debounce_fn_with_options(handle_resize, 100.0, DebounceOptions::default().max_wait(Some(100.0)));

    // window_event_listener(resize, move |_| {
    use_resize_observer(wrapper, move |_entries, _observer| {
        handle_resize_debounced();
    });

    view! {
        <div class="wrapper" node_ref=wrapper>
            <div class="display" style:padding=move || format!("{}px {}px", padding.get().1, padding.get().0)>
                <For each=move || 0..grid_size.get().1 key=|&y| y children=move |y| {
                    view! {
                        <div class="line">
                            <For each=move || 0..grid_size.get().0 key=|&x| x children=move |x| {
                                let cell = create_memo(move |_| grid.with(|g| g.get(&(x, y)).cloned()));
                                view! {
                                    <CellElement cell=cell />
                                }
                            }/>
                        </div>
                    }
                }/>
            </div>
        </div>
    }
}

#[component]
fn CellElement(cell: Memo<Option<Cell>>) -> impl IntoView {
    view! {
        {move || match cell.get() {
            Some(c) => view! {
                <span class="cell"
                    style:color=c.style.foreground.value()
                    style:background-color=c.style.background.value()
                    style:font-weight=if c.style.bold {"bold"} else {"normal"}
                    style:font-style=if c.style.italic {"italic"} else {"normal"}
                >
                    {c.char}
                </span>
            },
            None => view! {
                <span class="cell"></span>
            }
        }}
    }
}
