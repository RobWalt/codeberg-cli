use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::api::notification::notification_subject::NotificationSubject;
use crate::api::repository::Repository;

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationThread {
    id: usize,
    pinned: bool,
    repository: Repository,
    subject: NotificationSubject,
    unread: bool,
    updated_at: DateTime<Utc>,
    url: String,
}
