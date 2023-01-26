use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PullRequestMeta {
    merged: bool,
    merged_at: Option<String>,
}
