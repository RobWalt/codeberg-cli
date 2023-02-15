use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ValueEnum, Display, EnumIter, EnumString)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum NotifySubjectType {
    Issue,
    Pull,
    Commit,
    Repository,
}
impl NotifySubjectType {
    pub fn available_for_choosing() -> [Self; 4] {
        use NotifySubjectType::*;
        [Issue, Pull, Commit, Repository]
    }
}
