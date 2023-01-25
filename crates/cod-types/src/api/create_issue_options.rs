use serde::Serialize;

#[derive(Serialize)]
pub struct CreateIssueOption {
    title: String,
    body: String,
    assignees: Vec<String>,
    labels: Vec<usize>,
}

impl CreateIssueOption {
    pub fn new(title: String) -> Self {
        Self {
            title,
            body: Default::default(),
            assignees: Default::default(),
            labels: Default::default(),
        }
    }

    pub fn with_body(mut self, body: String) -> Self {
        self.body = body;
        self
    }

    pub fn with_assignees(mut self, assignees: Vec<String>) -> Self {
        self.assignees = assignees;
        self
    }

    pub fn with_labels(mut self, labels: Vec<usize>) -> Self {
        self.labels = labels;
        self
    }
}
