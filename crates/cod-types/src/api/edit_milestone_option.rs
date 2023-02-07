use serde::Serialize;

use crate::api::milestone::Milestone;
use crate::api::state_type::StateType;

#[derive(Debug, Clone, Serialize, Default)]
pub struct EditMilestoneOption {
    pub description: Option<String>,
    pub state: Option<StateType>,
    pub title: Option<String>,
    pub due_on: Option<String>,
}

impl EditMilestoneOption {
    pub fn from_milestone(milestone: &Milestone) -> Self {
        Self {
            description: milestone.description.clone(),
            due_on: milestone.due_on.clone(),
            state: Some(milestone.state),
            title: Some(milestone.title.clone()),
        }
    }
}
