use serde::Deserialize;

use crate::api::label::Label;

#[derive(Debug, Clone, Deserialize)]
pub struct Issue {
    pub title: String,
    pub number: usize,
    pub labels: Vec<Label>,
}
