use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};

use super::{LEAGUE, PREV_LEAGUE};

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

#[derive(Default, Debug, Enum, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum League {
    #[default]
    TmpStandard,
    TmpHardcore,
    TmpRuthless,
    TmpHardcoreRuthless,
    Standard,
    Hardcore,
    Ruthless,
    HardcoreRuthless,
    PrevStandard,
    PrevHardcore,
    PrevRuthless,
    PrevHardcoreRuthless,
}

impl ToString for League {
    fn to_string(&self) -> String {
        match self {
            League::TmpStandard => LEAGUE.to_string(),
            League::TmpHardcore => format!("Hardcore+{}", LEAGUE),
            League::TmpRuthless => format!("Ruthless+{}", LEAGUE),
            League::TmpHardcoreRuthless => format!("HC+Ruthless+{}", LEAGUE),
            League::Standard => "Standard".to_string(),
            League::Hardcore => "Hardcore".to_string(),
            League::Ruthless => "Ruthless".to_string(),
            League::HardcoreRuthless => "Hardcore+Ruthless".to_string(),
            League::PrevStandard => PREV_LEAGUE.to_string(),
            League::PrevHardcore => format!("Hardcore+{}", PREV_LEAGUE),
            League::PrevRuthless => format!("Ruthless+{}", PREV_LEAGUE),
            League::PrevHardcoreRuthless => format!("HC+Ruthless+{}", PREV_LEAGUE),
        }
    }
}
