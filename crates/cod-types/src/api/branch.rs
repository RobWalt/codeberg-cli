use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Branch {
    pub name: String,
}
