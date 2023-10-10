use super::cache::fetch_with_cache;
use super::filters::WhereInput;
use super::ninja_item::{Item, ItemEndpoint, ItemOrderby, ItemRaw, ItemWhere};
use super::orderby::OrderbyInput;
use futures::future;

async fn fetch_item_endpoint(league: &str, endpoint: &ItemEndpoint) -> ItemRaw {
    let endpoint_str = endpoint.to_string();
    let url = format!(
        "https://poe.ninja/api/data/itemoverview?league={}&type={}",
        league, endpoint_str
    );
    let mut items = reqwest::get(url)
        .await
        .unwrap_or_else(|_| panic!("could not fetch item data from endpoint: {}", endpoint_str))
        .json::<ItemRaw>()
        .await
        .unwrap_or_else(|_| panic!("could not parse item data from endpoint: {}", endpoint_str));

    // add endpoint information
    items.lines.iter_mut().for_each(|line| {
        line.endpoint = *endpoint;
    });
    items
}

async fn fetch_items(league: &str) -> Vec<Item> {
    // let items: ItemRaw =
    //     serde_json::from_str(include_str!("jewelry.json")).expect("failed to parse jewelry.json");

    let responses = future::join_all(
        [
            // General
            ItemEndpoint::Tattoo,
            ItemEndpoint::Omen,
            ItemEndpoint::DivinationCard,
            ItemEndpoint::Artifact,
            ItemEndpoint::Oil,
            ItemEndpoint::Incubator,
            // Equipment & Gems
            ItemEndpoint::UniqueWeapon,
            ItemEndpoint::UniqueArmour,
            ItemEndpoint::UniqueAccessory,
            ItemEndpoint::UniqueFlask,
            ItemEndpoint::UniqueJewel,
            ItemEndpoint::UniqueRelic,
            ItemEndpoint::SkillGem,
            ItemEndpoint::ClusterJewel,
            // Atlas
            ItemEndpoint::Map,
            ItemEndpoint::BlightedMap,
            ItemEndpoint::BlightRavagedMap,
            ItemEndpoint::ScourgedMap,
            ItemEndpoint::UniqueMap,
            ItemEndpoint::DeliriumOrb,
            ItemEndpoint::Invitation,
            ItemEndpoint::Scarab,
            ItemEndpoint::Memory,
            // Crafting
            ItemEndpoint::BaseType,
            ItemEndpoint::Fossil,
            ItemEndpoint::Resonator,
            ItemEndpoint::HelmetEnchant,
            ItemEndpoint::Beast,
            ItemEndpoint::Essence,
            ItemEndpoint::Vial,
        ]
        .iter()
        .map(|endpoint| async move { fetch_item_endpoint(league, endpoint).await }),
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

            // TODO: add gem level and quality to name
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
        // error will be bubbled up
        .unwrap();

    let mut items = if let Some(_where) = _where {
        _where.filter_recursive(&items)
    } else {
        items
    };

    ItemOrderby::orderby(&mut items, _orderby);

    items
}
