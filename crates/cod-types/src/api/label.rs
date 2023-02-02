use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Label {
    pub color: String,
    pub description: String,
    pub name: String,
    pub id: usize,
}
