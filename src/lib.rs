#![deny(missing_docs)]

//! Static web app for everything concerning equipment in Old School RuneScape.

pub mod model;
pub mod ui;

use seed::prelude::*;

#[doc(hidden)]
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", model::init, model::update, ui::view);
}
