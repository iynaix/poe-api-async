use super::cache::fetch_with_cache;
use super::filters::WhereInput;
use super::ninja_item::{Item, ItemOrderby, ItemRaw, ItemWhere};
use super::orderby::OrderbyInput;
use futures::future;

async fn fetch_item_endpoint(league: &str, endpoint: &str) -> ItemRaw {
    let url = format!(
        "https://poe.ninja/api/data/itemoverview?league={}&type={}",
        league, endpoint
    );
    reqwest::get(url)
        .await
        .unwrap_or_else(|_| panic!("could not fetch item data from endpoint: {}", endpoint))
        .json::<ItemRaw>()
        .await
        .unwrap_or_else(|_| panic!("could not parse item data from endpoint: {}", endpoint))
}

async fn fetch_items(league: &str) -> Vec<Item> {
    // let items: ItemRaw =
    //     serde_json::from_str(include_str!("jewelry.json")).expect("failed to parse jewelry.json");

    let endpoints = [
        // General
        "Tattoo",
        "Omen",
        "DivinationCard",
        "Artifact",
        "Oil",
        "Incubator",
        // Equipment & Gems
        "UniqueWeapon",
        "UniqueArmour",
        "UniqueAccessory",
        "UniqueFlask",
        "UniqueJewel",
        "UniqueRelic",
        "SkillGem",
        "ClusterJewel",
        // Atlas
        "Map",
        "BlightedMap",
        "BlightRavagedMap",
        "ScourgedMap",
        "UniqueMap",
        "DeliriumOrb",
        "Invitation",
        "Scarab",
        "Memory",
        // Crafting
        "BaseType",
        "Fossil",
        "Resonator",
        "HelmetEnchant",
        "Beast",
        "Essence",
        "Vial",
    ];

    let responses = future::join_all(
        endpoints
            .iter()
            .map(|&endpoint| async move { fetch_item_endpoint(league, endpoint).await }),
    )
    .await;

    let items = responses
        .into_iter()
        .fold(ItemRaw::default(), |mut acc, curr| {
            acc.lines.extend(curr.lines);
            acc
        });

    items
        .lines
        .iter()
        .map(|item| {
            let mut name = item.name.clone();

            let is_relic = item.details_id.ends_with("-relic");
            if is_relic {
                name = format!("{} (Relic)", &item.name);
            }

            // if (endpoint === "SkillGem") {
            //     const corrupted = Boolean(item.corrupted) ? " (Corrupted)" : ""
            //     name = `${item.name} (${item.gemLevel}/${item.gemQuality || 0}${corrupted})`
            // }

            Item {
                name,
                ..item.clone()
            }
        })
        .collect()
}

pub async fn get_items(_where: Option<ItemWhere>, _orderby: Vec<ItemOrderby>) -> Vec<Item> {
    let league = "Ancestor";

    let items = fetch_with_cache("item", league, || async { fetch_items(league).await })
        .await
        // error will be bubbled up from
        .unwrap();

    let mut items = if let Some(_where) = _where {
        _where.filter_recursive(&items)
    } else {
        items
    };

    ItemOrderby::orderby(&mut items, _orderby);

    items
}
