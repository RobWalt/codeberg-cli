use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UserInfo {
    pub username: String,
    pub followers_count: usize,
    pub following_count: usize,
}
