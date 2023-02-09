use std::fmt::Display;

use chrono::{DateTime, Utc};
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
    pub closed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
    pub merged_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}

impl Display for PullRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{} {}", self.number, self.title)
    }
}
