use anyhow::{anyhow, Result};
use seed::prelude::fetch;
use serde::Deserialize;

use super::{EquipmentStorage, Item};

macro_rules! query_string {
    (@rep $l:expr, $($ls:expr),*) => {
        concat!(r#"{"equipment."#, $l, r#"":{"$gt":0}},"#,
            query_string!(@rep $($ls),*))

    };
    (@rep $l:expr) => {
        concat!(r#"{"equipment."#, $l, r#"":{"$gt":0}}"#)
    };
    ($($ls:expr),*) => {
        // TODO exclusions: "id":{"$nin":["732"]},
        concat!(r#"{"duplicate":false,"$or":["#,
            query_string!(@rep $($ls),*),
            "]}")
    };
}

const QUERY_STRING: &str = query_string!(
    "attack_stab",
    "attack_slash",
    "attack_crush",
    "attack_magic",
    "attack_ranged",
    "defence_stab",
    "defence_slash",
    "defence_crush",
    "defence_magic",
    "defence_ranged",
    "melee_strength",
    "ranged_strength",
    "magic_damage",
    "prayer"
);

#[derive(Debug, Clone, Deserialize)]
struct QueryResponseMeta {
    total: usize,
}

#[derive(Debug, Clone, Deserialize)]
struct QueryResponse {
    #[serde(rename = "_items")]
    items: Vec<Item>,
    #[serde(rename = "_meta")]
    meta: QueryResponseMeta,
}

/// Manages the loading of items from paged json responses.
#[derive(Debug)]
pub struct DataLoader {
    storage: EquipmentStorage,
    count: usize,
    total: usize,
    next_page: usize,
}

impl DataLoader {
    async fn get_page(page: usize) -> Result<QueryResponse> {
        let url = create_url(page);
        let map = |e| anyhow!("Error fetching {}: {:?}", url, e);

        let response = fetch(&url).await.map_err(map)?;
        response.json().await.map_err(map)
    }

    /// Create a new DataLoader. Will load the first page of items to retrieve the total count.
    pub async fn new() -> Result<Self> {
        let first_page = Self::get_page(1).await?;

        let mut result = Self {
            storage: EquipmentStorage::default(),
            count: 0,
            total: first_page.meta.total,
            next_page: 2,
        };

        result.add_items(first_page.items);
        Ok(result)
    }

    fn add_items(&mut self, items: Vec<Item>) {
        let len = items.len();

        for item in items.into_iter() {
            self.storage[item.slot()].push(item)
        }

        self.count += len;
    }

    /// Returns `(loaded items, total items)`.
    pub fn progress(&self) -> (usize, usize) {
        (self.count, self.total)
    }

    /// Load the next page of items, returns true if all items are loaded.
    ///
    /// Should not be called again after it returned `true`.
    pub async fn load_next(&mut self) -> Result<bool> {
        debug_assert!(self.count < self.total);

        let page = Self::get_page(self.next_page).await?;
        self.next_page += 1;

        self.add_items(page.items);

        Ok(self.count == self.total)
    }

    /// Retrieve the completed [`EquipmentStorage`](struct.EquipmentStorage.html).
    ///
    /// Can only be called after [`load_next`](#method.load_next) returned `true`.
    pub fn storage(self) -> EquipmentStorage {
        assert!(self.count == self.total);
        self.storage
    }
}

fn create_url(page: usize) -> String {
    format!(
        "https://api.osrsbox.com/equipment?where={}&projection={}&page={}",
        QUERY_STRING,
        super::item::PROJECTION_STRING,
        page
    )
}
