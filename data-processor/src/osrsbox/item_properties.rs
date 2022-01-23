use data::{Attainability, Item};
use image::{bmp::BmpEncoder, DynamicImage, GenericImageView, ImageFormat};
use serde::Deserialize;

use super::{ItemEquipment, ItemWeapon};

/// [OSRSBox](https://www.osrsbox.com/) [`ItemProperties`](https://www.osrsbox.com/projects/osrsbox-db/#item-properties).
#[allow(clippy::struct_excessive_bools)]
#[derive(Deserialize)]
pub struct ItemProperties {
    /// Unique OSRS item ID number.
    pub id: u32,
    /// The name of the item.
    pub name: String,
    /// The last time (UTC) the item was updated (in ISO8601 date format).
    pub last_updated: String,
    /// If the item has incomplete wiki data.
    pub incomplete: bool,
    /// If the item is a members-only.
    pub members: bool,
    /// If the item is tradeable (between players and on the GE).
    pub tradeable: bool,
    /// If the item is tradeable (only on GE).
    pub tradeable_on_ge: bool,
    /// If the item is stackable (in inventory).
    pub stackable: bool,
    /// If the item is stacked, indicated by the stack count.
    pub stacked: Option<u32>,
    /// If the item is noted.
    pub noted: bool,
    /// If the item is noteable.
    pub noteable: bool,
    /// The linked ID of the actual item (if noted/placeholder).
    pub linked_id_item: Option<u32>,
    /// The linked ID of an item in noted form.
    pub linked_id_noted: Option<u32>,
    /// The linked ID of an item in placeholder form.
    pub linked_id_placeholder: Option<u32>,
    /// If the item is a placeholder.
    pub placeholder: bool,
    /// If the item is equipable (based on right-click menu entry).
    pub equipable: bool,
    /// If the item is equipable in-game by a player.
    pub equipable_by_player: bool,
    /// If the item is an equipable weapon.
    pub equipable_weapon: bool,
    /// The store price of an item.
    pub cost: u32,
    /// The low alchemy value of the item (cost * 0.4).
    pub lowalch: Option<u32>,
    /// The high alchemy value of the item (cost * 0.6).
    pub highalch: Option<u32>,
    /// The weight (in kilograms) of the item.
    pub weight: Option<f32>,
    /// The Grand Exchange buy limit of the item.
    pub buy_limit: Option<u32>,
    /// If the item is associated with a quest.
    pub quest_item: bool,
    /// Date the item was released (in ISO8601 format).
    pub release_date: Option<String>,
    /// If the item is a duplicate.
    pub duplicate: bool,
    /// The examine text for the item.
    pub examine: Option<String>,
    /// The item icon icon(in base64 encoding).
    pub icon: String,
    /// The OSRS Wiki name for the item.
    pub wiki_name: Option<String>,
    /// The OSRS Wiki URL (possibly including anchor link).
    pub wiki_url: Option<String>,
    /// The OSRS Wiki Exchange URL.
    pub wiki_exchange: Option<String>,
    /// The equipment bonuses of equipable armour/weapons.
    pub equipment: Option<ItemEquipment>,
    /// The weapon bonuses including attack speed, type and stance.
    pub weapon: Option<ItemWeapon>,
}

impl From<ItemProperties> for Item {
    fn from(item: ItemProperties) -> Self {
        assert!(!item.duplicate);

        let equipment = item.equipment.expect("Missing equipment stats.");
        let equip_slot = equipment.slot;
        let requirements = equipment.requirements();

        Self {
            name: item.name,
            members: item.members,
            weight: item.weight.expect("Missing weight."),
            wiki_url: item.wiki_url.expect("Missing wiki url."),
            icon_data: trim_icon(&item.icon).unwrap(),
            combat_stats: equipment.into(),
            weapon_data: item.weapon.map(Into::into),
            equip_slot,
            requirements,
            attainability: Attainability::new(item.tradeable),
        }
    }
}

fn trim_icon(data: &str) -> Result<Vec<u8>, String> {
    let bytes = base64::decode(data).map_err(|e| format!("Failed to decode base64: {}", e))?;

    let image = image::load_from_memory_with_format(&bytes, ImageFormat::Png)
        .map_err(|e| format!("Failed to decode PNG: {}", e))?;

    let (x, y, w, h) = find_dimensions(&image);
    let image = image.crop_imm(x, y, w, h);

    let mut output = Vec::new();
    let mut encoder = BmpEncoder::new(&mut output);
    encoder
        .encode(image.as_bytes(), w, h, image.color())
        .map_err(|e| format!("Failed to encode PNG: {}", e))?;

    Ok(output)
}

fn find_dimensions(image: &DynamicImage) -> (u32, u32, u32, u32) {
    let (w, h) = image.dimensions();

    let (mut x0, mut y0) = (u32::MAX, u32::MAX);
    let (mut x1, mut y1) = (0, 0);
    for x in 0..w {
        for y in 0..h {
            let pixel = image.get_pixel(x, y);
            if pixel[3] != 0 {
                x0 = x0.min(x);
                x1 = x1.max(x);
                y0 = y0.min(y);
                y1 = y1.max(y);
            }
        }
    }

    (x0, y0, x1 - x0, y1 - y0)
}
