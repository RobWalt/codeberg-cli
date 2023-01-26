use serde::Deserialize;

use crate::api::user::User;

#[derive(Clone, Debug, Deserialize)]
pub struct Repository {
    pub name: String,
    pub owner: User,
    pub stars_count: usize,
    pub ssh_url: String,
}
