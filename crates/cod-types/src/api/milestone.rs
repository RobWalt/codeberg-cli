use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::api::state_type::StateType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub id: usize,
    pub title: String,
    pub due_on: Option<DateTime<Utc>>,
    pub state: StateType,
    pub open_issues: usize,
    pub closed_issues: usize,
    pub description: Option<String>,
}

impl Display for Milestone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}
