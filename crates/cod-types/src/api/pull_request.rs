use serde::Deserialize;

use crate::api::label::Label;
use crate::api::state_type::StateType;

#[derive(Debug, Clone, Deserialize)]
pub struct PullRequest {
    pub title: String,
    pub number: usize,
    pub labels: Vec<Label>,
    pub state: StateType,
}
