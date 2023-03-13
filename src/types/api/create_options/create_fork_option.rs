use serde::Serialize;

#[derive(Serialize)]
pub struct CreateForkOption {
    name: Option<String>,
    organization: Option<String>,
}

impl CreateForkOption {
    pub fn same_repo_name() -> Self {
        Self {
            name: None,
            organization: None,
        }
    }
}
