use serde::{Deserialize, Serialize};

use crate::api::notification::notification_state_type::NotificationStateType;
use crate::api::notification::notification_type::NotifySubjectType;

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationSubject {
    html_url: String,
    latest_comment_html_url: String,
    latest_comment_url: String,
    state: NotificationStateType,
    title: String,
    #[serde(rename = "type")]
    notify_type: NotifySubjectType,
}
