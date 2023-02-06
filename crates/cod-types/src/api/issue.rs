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
}
