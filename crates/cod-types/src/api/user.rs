use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct User {
    pub id: usize,
    pub username: String,
    pub followers_count: usize,
    pub following_count: usize,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.username)
    }
}
