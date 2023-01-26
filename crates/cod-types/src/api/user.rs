use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct User {
    pub id: usize,
    pub username: String,
    pub followers_count: usize,
    pub following_count: usize,
}
