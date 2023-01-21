use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UserInfo {
    pub(crate) username: String,
}
