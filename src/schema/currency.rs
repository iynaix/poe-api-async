use futures::future;
use std::collections::HashMap;

use super::cache::fetch_with_cache;
use super::filters::WhereInput;
use super::ninja_common::League;
use super::ninja_currency::{
    Currency, CurrencyEndpoint, CurrencyOrderby, CurrencyRaw, CurrencyWhere,
};
use super::orderby::OrderbyInput;

async fn fetch_currency_endpoint(league: League, endpoint: &CurrencyEndpoint) -> CurrencyRaw {
    let endpoint_str = endpoint.to_string();
    let url = format!(
        "https://poe.ninja/api/data/currencyoverview?league={}&type={}",
        league.to_string(),
        endpoint_str
    );
    let mut currencies = reqwest::get(url)
        .await
        .unwrap_or_else(|_| {
            panic!(
                "could not fetch currency data from endpoint: {}",
                endpoint_str
            )
        })
        .json::<CurrencyRaw>()
        .await
        .unwrap_or_else(|_| {
            panic!(
                "could not parse currency data from endpoint: {}",
                endpoint_str
            )
        });

    // add endpoint information
    currencies.lines.iter_mut().for_each(|line| {
        line.endpoint = *endpoint;
    });
    currencies
}

async fn fetch_currencies(league: League) -> Vec<Currency> {
    // let currencies: CurrencyRaw = serde_json::from_str(include_str!("currencies.json"))
    //     .expect("failed to parse currencies.json");

    // fetch multiple requests and join them
    // https://stackoverflow.com/a/75590180

    let responses = future::join_all(
        [CurrencyEndpoint::Currency, CurrencyEndpoint::Fragment]
            .iter()
            .map(|endpoint| async move { fetch_currency_endpoint(league, endpoint).await }),
    )
    .await;

    let currencies = responses
        .into_iter()
        .fold(CurrencyRaw::default(), |mut acc, curr| {
            acc.lines.extend(curr.lines);
            acc.currency_details.extend(curr.currency_details);
            acc
        });

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

    currencies
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
        .collect()
}

pub async fn get_currencies(
    _where: Option<CurrencyWhere>,
    _orderby: Vec<CurrencyOrderby>,
    league: Option<League>,
) -> Vec<Currency> {
    let league = league.unwrap_or(League::TmpStandard);

    let currencies = fetch_with_cache("currency", league, || async {
        fetch_currencies(league).await
    })
    .await
    // error will be bubbled up
    .unwrap();

    let mut currencies = if let Some(_where) = _where {
        _where.filter_recursive(&currencies)
    } else {
        currencies
    };

    CurrencyOrderby::orderby(&mut currencies, _orderby);

    currencies
}
