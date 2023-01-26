use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CreateRepoOption {
    default_branch: String,
    description: String,
    name: String,
    private: bool,
    readme: String,
}

impl CreateRepoOption {
    pub fn new(name: String) -> Self {
        Self {
            default_branch: String::from("main"),
            description: Default::default(),
            name,
            private: true,
            readme: Default::default(),
        }
    }

    pub fn with_default_branch(mut self, default_branch: String) -> Self {
        self.default_branch = default_branch;
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn private(mut self) -> Self {
        self.private = true;
        self
    }

    pub fn public(mut self) -> Self {
        self.private = false;
        self
    }

    pub fn with_readme(mut self, readme: String) -> Self {
        self.readme = readme;
        self
    }
}
