use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CreateMilestoneOption {
    title: String,
    due_on: Option<String>,
    description: Option<String>,
}

impl CreateMilestoneOption {
    pub fn new(title: String) -> Self {
        Self {
            title,
            due_on: Default::default(),
            description: Default::default(),
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description.replace(description);
        self
    }
}
