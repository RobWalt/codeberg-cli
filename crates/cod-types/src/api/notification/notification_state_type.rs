use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ValueEnum, Display, EnumIter, EnumString)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum NotificationStateType {
    Unread,
    Read,
    Pinned,
}

impl NotificationStateType {
    pub fn available_for_choosing() -> [Self; 3] {
        use NotificationStateType::*;
        [Unread, Read, Pinned]
    }
}
