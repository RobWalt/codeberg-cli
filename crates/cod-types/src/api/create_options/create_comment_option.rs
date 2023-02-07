use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CreateCommentOption {
    body: String,
}

impl CreateCommentOption {
    pub fn new(body: String) -> Self {
        Self { body }
    }
}
