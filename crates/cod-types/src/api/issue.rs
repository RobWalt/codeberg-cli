use serde::Deserialize;

use crate::api::label::Label;
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
}
