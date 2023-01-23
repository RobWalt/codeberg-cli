use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub username: String,
    pub followers_count: usize,
    pub following_count: usize,
}
