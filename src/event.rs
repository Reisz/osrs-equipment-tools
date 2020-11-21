//! Event handling.

use seed::prelude::*;

use super::{
    model::{Data, Model},
    osrsbox::DataLoader,
};

/// Possible events.
pub enum Msg {
    /// Data loading has progressed. Keeps the DataLoader for lifetime purposes.
    LoadingProgress(DataLoader),
}

/// Reacts to events.
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::LoadingProgress(mut loader) => {
            if loader.is_done() {
                model.data = Some(Data::Done(loader.storage()));
            } else {
                let (count, total) = loader.progress();
                model.data = Some(Data::Loading(count, total));
                orders.perform_cmd(async {
                    loader.load_next().await.unwrap();
                    Msg::LoadingProgress(loader)
                });
            }
        }
    }
}
