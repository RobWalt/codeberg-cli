use serde::Serialize;

use crate::api::issue::Issue;
use crate::api::state_type::StateType;

#[derive(Debug, Clone, Serialize, Default)]
pub struct EditIssueOptions {
    pub assignees: Option<Vec<String>>,
    pub body: Option<String>,
    pub state: Option<StateType>,
    pub title: Option<String>,
}

impl EditIssueOptions {
    pub fn from_issue(issue: &Issue) -> Self {
        Self {
            assignees: issue.assignees.as_ref().map(|assignees| {
                assignees
                    .iter()
                    .map(|assignee| assignee.username.to_owned())
                    .collect::<Vec<_>>()
            }),
            body: Some(issue.body.clone()),
            state: Some(issue.state),
            title: Some(issue.title.clone()),
        }
    }
}
