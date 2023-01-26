use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PullRequestMeta {
    pub merged: bool,
    pub merged_at: Option<String>,
}
