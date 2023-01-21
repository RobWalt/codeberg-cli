use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Label {
    pub(crate) name: String,
}
