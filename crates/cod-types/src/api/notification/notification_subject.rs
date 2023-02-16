use serde::{Deserialize, Serialize};

use crate::api::notification::notification_state_type::NotificationStateTypeApi;
use crate::api::notification::notification_type::NotificationSubjectType;

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationSubject {
    pub html_url: String,
    pub latest_comment_html_url: String,
    pub latest_comment_url: String,
    pub state: NotificationStateTypeApi,
    pub title: String,
    #[serde(rename = "type")]
    pub notify_type: NotificationSubjectType,
}
