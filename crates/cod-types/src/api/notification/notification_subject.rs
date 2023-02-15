use serde::{Deserialize, Serialize};

use crate::api::notification::notification_state_type::NotificationStateType;
use crate::api::notification::notification_type::NotifySubjectType;

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationSubject {
    pub html_url: String,
    pub latest_comment_html_url: String,
    pub latest_comment_url: String,
    pub state: NotificationStateType,
    pub title: String,
    #[serde(rename = "type")]
    pub notify_type: NotifySubjectType,
}
