use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ValueEnum, Display, EnumIter, EnumString)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum StateType {
    Closed,
    Open,
    All,
}

impl StateType {
    pub fn available_for_choosing() -> [StateType; 2] {
        use StateType::*;
        [Closed, Open]
    }
}
