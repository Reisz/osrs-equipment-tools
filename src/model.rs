//! Datastructures for representing the applicaton state in [`Model`].

use seed::prelude::*;

use crate::{
    event::Msg,
    osrsbox::{DataLoader, EquipmentStorage},
};

/// Item data status.
#[allow(clippy::large_enum_variant)] // Done is the main state of the enum
pub enum Data {
    /// Item data is loading. Loader is stored in messages
    Loading(usize, usize),
    /// Item data is loaded and accessible
    Done(EquipmentStorage),
}

/// The application state.
#[derive(Default)]
pub struct Model {
    /// Item database
    pub data: Option<Data>,
}

/// Initialize the model and start item data loading process.
pub fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async { Msg::LoadingProgress(DataLoader::new().await.unwrap()) });

    Model::default()
}
