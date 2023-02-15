use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::api::notification::notification_subject::NotificationSubject;
use crate::api::repository::Repository;

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationThread {
    pub id: usize,
    pub pinned: bool,
    pub repository: Repository,
    pub subject: NotificationSubject,
    pub unread: bool,
    pub updated_at: DateTime<Utc>,
    pub url: String,
}

impl Display for NotificationThread {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.subject.title.as_str())
    }
}
