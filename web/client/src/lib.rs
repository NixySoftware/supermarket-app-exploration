use leptos::*;
use supermarket_web_app::App;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
