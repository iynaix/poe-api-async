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
use ninja_common::League;
use ninja_currency::{Currency, CurrencyOrderby, CurrencyWhere};
use ninja_item::{Item, ItemOrderby, ItemWhere};
use orderby::Orderby;

pub struct QueryRoot;

static LEAGUE: &str = "Ancestor";
static PREV_LEAGUE: &str = "Crucible";

#[Object]
impl QueryRoot {
    async fn currency<'a>(
        &self,
        _ctx: &Context<'a>,
        _where: Option<CurrencyWhere>,
        _orderby: Option<Vec<CurrencyOrderby>>,
        league: Option<League>,
    ) -> Vec<Currency> {
        let orderby_arr = _orderby.unwrap_or(vec![CurrencyOrderby::name(Orderby::Asc)]);

        get_currencies(_where, orderby_arr, league).await
    }

    async fn item<'a>(
        &self,
        _ctx: &Context<'a>,
        _where: Option<ItemWhere>,
        _orderby: Option<Vec<ItemOrderby>>,
        league: Option<League>,
    ) -> Vec<Item> {
        let orderby_arr = _orderby.unwrap_or(vec![ItemOrderby::name(Orderby::Asc)]);

        get_items(_where, orderby_arr, league).await
    }
}
