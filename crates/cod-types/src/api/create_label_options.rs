use serde::Serialize;

// TODO: Use color crate
#[derive(Serialize)]
pub struct CreateLabelOption {
    color: String,
    description: String,
    name: String,
}

impl CreateLabelOption {
    pub fn new(name: String) -> Self {
        Self {
            color: String::from("#000000"),
            description: String::from("No description"),
            name,
        }
    }

    pub fn with_color(mut self, color: String) -> Self {
        self.color = color;
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
}
