#![deny(missing_docs)]

//! Static web app for everything concerning equipment in Old School RuneScape.

mod model;
mod ui;

use seed::prelude::*;

fn main() {
    App::start("app", model::init, model::update, ui::view);
}
