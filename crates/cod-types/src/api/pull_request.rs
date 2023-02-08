use std::fmt::Display;

use serde::Deserialize;

use crate::api::label::Label;
use crate::api::milestone::Milestone;
use crate::api::state_type::StateType;
use crate::api::user::User;

#[derive(Debug, Clone, Deserialize)]
pub struct PullRequest {
    pub title: String,
    pub body: String,
    pub number: usize,
    pub labels: Vec<Label>,
    pub state: StateType,
    pub assignees: Option<Vec<User>>,
    pub milestone: Option<Milestone>,
    pub merged: bool,
}

impl Display for PullRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{} {}", self.number, self.title)
    }
}
