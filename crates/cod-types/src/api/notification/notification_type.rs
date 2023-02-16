use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ValueEnum, Display, EnumIter, EnumString)]
#[strum(serialize_all = "lowercase")]
#[serde(rename(serialize = "lowercase"))]
pub enum NotificationSubjectType {
    Issue,
    Pull,
    Commit,
    Repository,
}
impl NotificationSubjectType {
    pub fn available_for_choosing() -> [Self; 4] {
        use NotificationSubjectType::*;
        [Issue, Pull, Commit, Repository]
    }
}
