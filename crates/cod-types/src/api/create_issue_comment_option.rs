use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CreateIssueCommentOption {
    body: String,
}

impl CreateIssueCommentOption {
    pub fn new(body: String) -> Self {
        Self { body }
    }
}
