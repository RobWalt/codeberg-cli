use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Label {
    pub name: String,
    pub id: usize,
}
