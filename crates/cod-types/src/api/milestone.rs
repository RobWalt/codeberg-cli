use serde::{Deserialize, Serialize};

use crate::api::state_type::StateType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub id: usize,
    pub title: String,
    pub due_on: Option<String>,
    pub state: StateType,
    pub open_issues: usize,
    pub closed_issues: usize,
    pub description: Option<String>,
}
