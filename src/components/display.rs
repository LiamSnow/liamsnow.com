use crate::models::grid::{Cell, Coord, CELL_WIDTH, LINE_HEIGHT};
use leptos::html::Div;
use leptos::*;
use leptos_use::{
    use_debounce_fn_with_options, use_element_size, use_resize_observer, DebounceOptions,
    UseElementSizeReturn,
};

#[component]
pub fn Display(
    grid: Memo<Vec<Cell>>,
    grid_size: RwSignal<Coord>,
    #[prop(into)] on_click: Callback<Coord>
) -> impl IntoView {
    let wrapper = NodeRef::<Div>::new();
    let UseElementSizeReturn { width, height } = use_element_size(wrapper);

    let handle_resize = move || {
        grid_size.update(|size| {
            *size = (
                (width.get() / CELL_WIDTH).floor() as usize,
                (height.get() / LINE_HEIGHT).floor() as usize,
            )
        });
    };
    let handle_resize_debounced = use_debounce_fn_with_options(
        handle_resize,
        100.0,
        DebounceOptions::default().max_wait(Some(200.0)),
    );

    // window_event_listener(resize, move |_| {
    use_resize_observer(wrapper, move |_entries, _observer| {
        handle_resize_debounced();
    });

    view! {
        <div class="wrapper" node_ref=wrapper>
            <div class="display"
                style:grid-template-columns=move || {
                    format!("repeat({}, 1fr)", grid_size.get().0)
                }
                style:grid-template-rows=move || {
                    format!("repeat({}, 1fr)", grid_size.get().1)
                }
            >
                <For each=move || grid.get()
                     key=|cell| *cell
                     children=move |cell| {
                        view! {
                            <span class="cell"
                                style:grid-row=cell.coord.1+1
                                style:grid-column=cell.coord.0+1
                                style:color=cell.style.foreground.value()
                                style:background-color=cell.style.background.value()
                                style:font-weight=if cell.style.bold {"bold"} else {"normal"}
                                style:font-style=if cell.style.italic {"italic"} else {"normal"}
                                on:click=move |_| { on_click(cell.coord) }
                            >
                                {cell.char}
                            </span>
                        }
                    }
                />
            </div>
        </div>
    }
}
