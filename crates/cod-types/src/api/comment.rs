use serde::Deserialize;

use crate::api::user::User;

#[derive(Debug, Deserialize)]
pub struct Comment {
    pub body: String,
    pub created_at: String,
    pub id: usize,
    pub user: User,
}
