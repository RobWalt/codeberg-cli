use serde::Serialize;

use crate::types::api::issue::Issue;
use crate::types::api::state_type::StateType;

#[derive(Debug, Clone, Serialize, Default)]
pub struct EditIssueOption {
    pub assignees: Option<Vec<String>>,
    pub body: Option<String>,
    pub state: Option<StateType>,
    pub title: Option<String>,
}

impl EditIssueOption {
    pub fn from_issue(issue: &Issue) -> Self {
        Self {
            assignees: issue.assignees.as_ref().map(|assignees| {
                assignees
                    .iter()
                    .map(|assignee| assignee.username.to_owned())
                    .collect::<Vec<_>>()
            }),
            body: Some(issue.body.clone()),
            state: issue
                .pull_request
                .as_ref()
                .map(|pr_meta| (!pr_meta.merged).then_some(issue.state))
                .unwrap_or(Some(issue.state)),
            title: Some(issue.title.clone()),
        }
    }
}
