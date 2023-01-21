use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RepoInfo {
    pub name: String,
    pub stars_count: isize,
}
