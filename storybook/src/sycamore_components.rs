use sycamore::prelude::*;
use ui_components_sycamore::pages::game_page::GamePage;
use wasm_bindgen::prelude::*;
use web_sys::Node;

#[wasm_bindgen]
pub fn game_page_component(parent: Node) {
    sycamore::render_to(
        |cx| {
            view! { cx,
                GamePage
            }
        },
        &parent,
    );
}
