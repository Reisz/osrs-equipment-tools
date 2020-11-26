use osrs_equipment_tools::osrsbox::{Item, PROJECTION_STRING, QUERY_STRING};
use serde::Deserialize;

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

fn get_page(items: &mut Vec<Item>, page: usize) -> usize {
    let response = reqwest::blocking::get(&format!(
        "https://api.osrsbox.com/equipment?where={}&projection={}&max_results=50&page={}",
        QUERY_STRING, PROJECTION_STRING, page
    ))
    .unwrap();
    let mut response: QueryResponse = response.json().unwrap();
    items.append(&mut response.items);
    response.meta.total
}

fn main() {
    let mut items = Vec::new();

    let total = get_page(&mut items, 1);
    let mut next_page = 2;
    while items.len() < total {
        get_page(&mut items, next_page);
        next_page += 1;

        println!("{}/{}", items.len(), total);
    }

    items.sort_unstable_by_key(|i| i.id());

    let mut exclude = Vec::new();
    for item in &items {
        let mut is_ok = true;

        for suffix in &[
            "(l)",
            "(uncharged)",
            "(empty)",
            "(u)",
            "(inactive)",
            "(full)",
            "(basic)",
            "(attuned)",
            "(perfected)",
            "(t)",
            "(g)",
            "(or)",
            "(dark)",
            "(light)",
            "(dusk)",
            "100",
            "75",
            "50",
            "25",
            "0",
            "(p)",
            "(p+)",
            "(p++)",
            "(kp)",
            "(nz)",
            "heraldic helm"
        ] {
            if item.name().ends_with(suffix) {
                is_ok = false;
                break;
            }
        }

        if is_ok {
            for infix in &["(h", "fire arrow", "slayer helmet"] {
                if item.name().contains(infix) {
                    is_ok = false;
                    break;
                }
            }
        }

        if is_ok {
            for prefix in &[
                "Uncharged",
                "Twisted",
                "Light infinity",
                "Dark infinity",
                "Gilded",
                "Broodoo shield (",
                "Black mask (",
                "Pharaoh's sceptre (",
                "Lunar staff -",
                "Combat bracelet(",
                "Amulet of glory(",
                "Volcanic abyssal whip",
                "Frozen abyssal whip",
                "Deadman's",
                "Trailblazer",
                "Vesta's",
                "Statius's",
                "Morrigan's",
                "Zuriel's"
            ] {
                if item.name().starts_with(prefix) {
                    is_ok = false;
                    break;
                }
            }
        }

        if is_ok {
            if item.wiki_url().contains("kiteshield_(") {
                    is_ok = false;
            }
        }

        if is_ok {
            println!("{} {}", item.name(), item.wiki_url());
        } else {
            exclude.push(item.id().to_string());
        }
    }

    println!("{}", exclude.join(", "));
}