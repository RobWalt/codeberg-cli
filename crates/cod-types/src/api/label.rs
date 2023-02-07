use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Label {
    pub color: String,
    pub description: String,
    pub name: String,
    pub id: usize,
}

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
