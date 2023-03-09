use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::types::api::user::User;

#[derive(Debug, Deserialize)]
pub struct Comment {
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub id: usize,
    pub user: User,
}
