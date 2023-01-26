use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CreatePullRequestOption {
    assignees: Vec<String>,
    base: String,
    body: String,
    head: String,
    labels: Vec<usize>,
    title: String,
}

impl CreatePullRequestOption {
    pub fn new(title: String, from: String, to: String) -> Self {
        Self {
            assignees: Default::default(),
            base: from,
            body: Default::default(),
            head: to,
            labels: Default::default(),
            title,
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
}
