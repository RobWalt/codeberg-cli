use serde::Deserialize;

use crate::types::api::label::Label;

#[derive(Debug, Clone, Deserialize)]
pub struct Issue {
    pub(crate) title: String,
    pub(crate) number: usize,
    pub(crate) labels: Vec<Label>,
}
