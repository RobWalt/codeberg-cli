use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CreatePullRequestOption {
    assignees: Vec<String>,
    base: String,
    body: String,
    head: String,
    labels: Vec<usize>,
    title: String,
    milestone: Option<usize>,
}

impl CreatePullRequestOption {
    pub fn new(title: String, from: String, to: String) -> Self {
        Self {
            title,
            head: from,
            base: to,
            assignees: Default::default(),
            body: Default::default(),
            labels: Default::default(),
            milestone: Default::default(),
        }
    }

    pub fn with_assignees(mut self, assignees: Vec<String>) -> Self {
        self.assignees = assignees;
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.body = description;
        self
    }

    pub fn with_labels(mut self, labels: Vec<usize>) -> Self {
        self.labels = labels;
        self
    }

    pub fn with_milestone(mut self, milestone_id: usize) -> Self {
        self.milestone.replace(milestone_id);
        self
    }
}
