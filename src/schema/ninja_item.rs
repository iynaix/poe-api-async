use async_graphql::SimpleObject;
use poe_api_derive::GQLModel;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
// use serde_json::Value;

use super::{filters::FilterInput, ninja_common::Sparkline};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemRaw {
    pub lines: Vec<Item>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, SimpleObject, GQLModel)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    #[gql(where, orderby)]
    pub name: String,
    pub icon: Option<String>,
    #[gql(where, orderby)]
    pub level_required: Option<i32>,
    #[gql(where, orderby)]
    pub base_type: Option<String>,
    #[gql(where, orderby)]
    pub links: Option<i32>,
    pub item_class: i32,
    #[gql(where)]
    pub gem_level: Option<i32>,
    #[gql(where)]
    pub gem_quality: Option<i32>,
    pub sparkline: Sparkline,
    pub low_confidence_sparkline: Sparkline,
    #[gql(where)]
    pub implicit_modifiers: Vec<Modifier>,
    #[gql(where)]
    pub explicit_modifiers: Vec<Modifier>,
    pub flavour_text: Option<String>,
    #[gql(where, orderby)]
    pub item_type: Option<String>,
    #[gql(where, orderby)]
    pub chaos_value: f64,
    pub exalted_value: f64,
    #[gql(where, orderby)]
    pub divine_value: f64,
    pub count: i32,
    pub details_id: String,
    // pub trade_info: Vec<Value>,
    pub listing_count: i32,
    #[gql(where, orderby)]
    pub variant: Option<String>,
    // added on
    #[serde(default)]
    #[gql(where)]
    pub corrupted: bool,
    #[serde(default)]
    #[gql(where)]
    pub endpoint: String,
}

// needed to dedupe items for recursive filters
impl Hash for Item {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Item {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Modifier {
    pub text: String,
    pub optional: bool,
}
