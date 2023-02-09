use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::api::label::Label;
use crate::api::milestone::Milestone;
use crate::api::pull_request_meta::PullRequestMeta;
use crate::api::state_type::StateType;
use crate::api::user::User;

#[derive(Debug, Clone, Deserialize)]
pub struct Issue {
    pub title: String,
    pub number: usize,
    pub labels: Vec<Label>,
    pub assignees: Option<Vec<User>>,
    pub body: String,
    pub state: StateType,
    pub pull_request: Option<PullRequestMeta>,
    pub milestone: Option<Milestone>,
    pub closed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}

impl Display for Issue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{} {}", self.number, self.title)
    }
}
