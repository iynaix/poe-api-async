use super::cache::fetch_with_cache;
use super::filters::WhereInput;
use super::ninja_item::{Item, ItemOrderby, ItemRaw, ItemWhere};
use super::orderby::OrderbyInput;

async fn fetch_items(
    _where: Option<ItemWhere>,
    _orderby: Vec<ItemOrderby>,
    league: &str,
) -> Vec<Item> {
    let endpoint = "UniqueAccessory";
    let url = format!(
        "https://poe.ninja/api/data/itemoverview?league={}&type={}",
        league, endpoint
    );
    let items = reqwest::get(url)
        .await
        .expect("could not fetch item data")
        .json::<ItemRaw>()
        .await
        .expect("could not parse item data");

    // let items: ItemRaw =
    //     serde_json::from_str(include_str!("jewelry.json")).expect("failed to parse jewelry.json");

    let items: Vec<_> = items
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
        .collect();

    let mut items = if let Some(_where) = _where {
        _where.filter_recursive(&items)
    } else {
        items
    };

    ItemOrderby::orderby(&mut items, _orderby);

    items
}

pub async fn get_items(_where: Option<ItemWhere>, _orderby: Vec<ItemOrderby>) -> Vec<Item> {
    let league = "Ancestor";

    fetch_with_cache("item", league, || async {
        fetch_items(_where, _orderby, league).await
    })
    .await
    .expect("failed to fetch item data")
}
