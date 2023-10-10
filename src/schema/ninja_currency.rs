use async_graphql::{Enum, SimpleObject};
use poe_api_derive::GQLModel;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

use super::{
    filters::FilterInput,
    ninja_common::{Sparkline, SparklineOptional},
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyRaw {
    pub lines: Vec<Currency>,
    pub currency_details: Vec<CurrencyDetail>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, SimpleObject, GQLModel)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    pub currency_type_name: String,
    pub pay: Option<Pay>,
    pub receive: Option<Receive>,
    pub pay_spark_line: SparklineOptional,
    pub receive_spark_line: Sparkline,
    #[serde(rename = "chaosEquivalent")]
    #[gql(where, orderby)]
    pub chaos_value: f64,
    pub low_confidence_pay_spark_line: SparklineOptional,
    pub low_confidence_receive_spark_line: Sparkline,
    pub details_id: String,
    // will be merged with currency_details
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    #[gql(where, orderby)]
    pub name: String,
    #[serde(default)]
    pub trade_id: Option<String>,
    // added on
    #[serde(default)]
    #[gql(where, orderby)]
    pub divine_value: f64,
    #[serde(default)]
    // TODO: #[gql(where)]
    pub endpoint: CurrencyEndpoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Pay {
    pub id: i32,
    #[serde(rename = "league_id")]
    pub league_id: i32,
    #[serde(rename = "pay_currency_id")]
    pub pay_currency_id: i32,
    #[serde(rename = "get_currency_id")]
    pub get_currency_id: i32,
    #[serde(rename = "sample_time_utc")]
    pub sample_time_utc: String,
    pub count: i32,
    pub value: f64,
    #[serde(rename = "data_point_count")]
    pub data_point_count: i32,
    #[serde(rename = "includes_secondary")]
    pub includes_secondary: bool,
    #[serde(rename = "listing_count")]
    pub listing_count: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Receive {
    pub id: i32,
    #[serde(rename = "league_id")]
    pub league_id: i32,
    #[serde(rename = "pay_currency_id")]
    pub pay_currency_id: i32,
    #[serde(rename = "get_currency_id")]
    pub get_currency_id: i32,
    #[serde(rename = "sample_time_utc")]
    pub sample_time_utc: String,
    pub count: i32,
    pub value: f64,
    #[serde(rename = "data_point_count")]
    pub data_point_count: i32,
    #[serde(rename = "includes_secondary")]
    pub includes_secondary: bool,
    #[serde(rename = "listing_count")]
    pub listing_count: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyDetail {
    pub id: i32,
    pub icon: Option<String>,
    pub name: String,
    pub trade_id: Option<String>,
}

// impl PartialOrd for Currency {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         self.name.partial_cmp(&other.name)
//     }
// }

// impl Ord for Currency {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.name.cmp(&other.name)
//     }
// }

// needed to dedupe currency for recursive filters
impl Hash for Currency {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Currency {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Currency {}

#[derive(Default, Debug, Enum, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum CurrencyEndpoint {
    #[default]
    Currency,
    Fragment,
}

impl ToString for CurrencyEndpoint {
    fn to_string(&self) -> String {
        match self {
            CurrencyEndpoint::Currency => "Currency",
            CurrencyEndpoint::Fragment => "Fragment",
        }
        .to_string()
    }
}
