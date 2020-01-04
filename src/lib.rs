#![recursion_limit = "256"]

#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate itertools;

pub mod component;
mod model;
mod state;

use component::Map;
use stdweb::web::{document, INonElementParentNode};
use wasm_bindgen::prelude::*;
use yew::App;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn mount(id: &str) {
    yew::initialize();
    let app = App::<Map>::new();
    app.mount(document().get_element_by_id(id).unwrap());
    yew::run_loop();
}
