use serde::Deserialize;

use crate::api::label::Label;

#[derive(Debug, Clone, Deserialize)]
pub struct PullRequest {
    pub title: String,
    pub number: usize,
    pub labels: Vec<Label>,
}
