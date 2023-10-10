use async_graphql::{Context, Object};

mod cache;
mod currency;
mod filters;
mod item;
mod ninja_common;
mod ninja_currency;
mod ninja_item;
mod orderby;

use currency::get_currencies;
use item::get_items;
use ninja_currency::{Currency, CurrencyOrderby, CurrencyWhere};
use ninja_item::{Item, ItemOrderby, ItemWhere};
use orderby::{Orderby, OrderbyInput};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn currency<'a>(
        &self,
        _ctx: &Context<'a>,
        _where: Option<CurrencyWhere>,
        _orderby: Option<CurrencyOrderby>,
    ) -> Vec<Currency> {
        let orderby_arr: Vec<CurrencyOrderby> = match _orderby {
            // default Value
            None => vec![CurrencyOrderby {
                name: Some(Orderby::Asc),
                ..Default::default()
            }],
            Some(_orderby) => OrderbyInput::to_orderby_vec(&_orderby),
        };

        get_currencies(_where, orderby_arr).await
    }

    async fn item<'a>(
        &self,
        _ctx: &Context<'a>,
        _where: Option<ItemWhere>,
        _orderby: Option<ItemOrderby>,
    ) -> Vec<Item> {
        let orderby_arr: Vec<ItemOrderby> = match _orderby {
            // default Value
            None => vec![ItemOrderby {
                name: Some(Orderby::Asc),
                ..Default::default()
            }],
            Some(_orderby) => OrderbyInput::to_orderby_vec(&_orderby),
        };

        get_items(_where, orderby_arr).await
    }
}
