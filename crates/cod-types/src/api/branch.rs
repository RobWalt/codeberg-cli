use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Branch {
    pub name: String,
}

impl Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
