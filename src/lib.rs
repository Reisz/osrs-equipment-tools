#![deny(missing_docs)]

//! Static web app for everything concerning equipment in Old School RuneScape.

pub mod event;
pub mod model;
pub mod osrsbox;
pub mod ui;

use seed::prelude::*;

#[doc(hidden)]
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", model::init, event::update, ui::view);
}
