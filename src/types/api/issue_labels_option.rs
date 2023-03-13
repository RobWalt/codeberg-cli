use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct IssueLabelsOption {
    pub labels: Vec<usize>,
}
