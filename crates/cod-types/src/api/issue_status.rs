use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ValueEnum, Display)]
#[strum(serialize_all = "lowercase")]
pub enum IssueStatus {
    Closed,
    Open,
    All,
}
