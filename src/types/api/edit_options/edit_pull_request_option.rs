use serde::Serialize;

use crate::types::api::pull_request::PullRequest;
use crate::types::api::state_type::StateType;

#[derive(Debug, Clone, Serialize, Default)]
pub struct EditPullRequestOption {
    pub assignees: Option<Vec<String>>,
    pub body: Option<String>,
    pub state: Option<StateType>,
    pub title: Option<String>,
    pub labels: Option<Vec<usize>>,
}

impl EditPullRequestOption {
    pub fn from_pull_request(pr: &PullRequest) -> Self {
        Self {
            assignees: pr.assignees.as_ref().map(|assignees| {
                assignees
                    .iter()
                    .map(|assignee| assignee.username.to_owned())
                    .collect::<Vec<_>>()
            }),
            body: Some(pr.body.clone()),
            state: (!pr.merged).then_some(pr.state),
            title: Some(pr.title.clone()),
            labels: Some(pr.labels.iter().map(|label| label.id).collect::<Vec<_>>()),
        }
    }
}
