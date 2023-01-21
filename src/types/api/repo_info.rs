use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RepoInfo {
    pub(crate) name: String,
    pub(crate) stars_count: isize,
}
