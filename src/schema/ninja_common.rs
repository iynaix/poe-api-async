use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Sparkline {
    pub data: Vec<Option<f64>>,
    pub total_change: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct SparklineOptional {
    pub data: Vec<Option<f64>>,
    pub total_change: f64,
}
