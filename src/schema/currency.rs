use std::collections::HashMap;

use super::filters::WhereInput;
use super::ninja_currency::{Currency, CurrencyOrderby, CurrencyRaw, CurrencyWhere};
use super::orderby::OrderbyInput;

pub async fn fetch_currencies(
    _where: Option<CurrencyWhere>,
    _orderby: Vec<CurrencyOrderby>,
) -> Vec<Currency> {
    let league = "Ancestor";
    let endpoint = "Currency";
    let url = format!(
        "https://poe.ninja/api/data/currencyoverview?league={}&type={}",
        league, endpoint
    );
    let currencies = reqwest::get(url)
        .await
        .expect("could not fetch currency data")
        .json::<CurrencyRaw>()
        .await
        .expect("could not parse currency data");

    // let currencies: CurrencyRaw = serde_json::from_str(include_str!("currencies.json"))
    //     .expect("failed to parse currencies.json");

    let mut divine_price = 0.0;

    let lines_by_type: HashMap<_, _> = currencies
        .lines
        .into_iter()
        .map(|line| {
            if line.currency_type_name == "Divine Orb" {
                divine_price = line.chaos_value;
            }
            (line.currency_type_name.to_string(), line)
        })
        .collect();

    let currencies: Vec<_> = currencies
        .currency_details
        .into_iter()
        .filter_map(|detail| {
            if let Some(line) = lines_by_type.get(&detail.name) {
                let id = detail
                    .trade_id
                    .as_ref()
                    .unwrap_or(&line.details_id)
                    .to_string();

                Some(Currency {
                    id,
                    divine_value: line.chaos_value / divine_price,
                    icon: detail.icon,
                    name: detail.name,
                    ..line.clone()
                })
            } else {
                None
            }
        })
        .collect();

    let mut currencies = if let Some(_where) = _where {
        _where.filter_recursive(&currencies)
    } else {
        currencies
    };

    CurrencyOrderby::orderby(&mut currencies, _orderby);

    currencies
}
