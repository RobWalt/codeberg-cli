use serde::Serialize;

use crate::api::label::Label;

#[derive(Debug, Clone, Serialize, Default)]
pub struct EditLabelOption {
    pub name: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
}

impl EditLabelOption {
    pub fn from_label(label: &Label) -> Self {
        Self {
            name: Some(label.name.to_string()),
            description: Some(label.description.to_string()),
            color: Some(label.color.to_string()),
        }
    }
}
