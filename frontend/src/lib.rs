#![recursion_limit = "256"]

pub(crate) mod app;
pub(crate) mod fetch;
pub(crate) mod pages;
pub(crate) mod route;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
}
