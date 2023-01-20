use std::ops::{Deref, DerefMut};
use std::path::Path;

use derive_new::new;

use crate::paths::token_path;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, new)]
pub struct Token(pub(crate) String);

impl Deref for Token {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Token {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Token {
    fn read_from_path(path: &Path) -> anyhow::Result<Self> {
        Ok(Self(std::fs::read_to_string(path)?))
    }

    pub fn read_from_data_dir() -> anyhow::Result<Self> {
        Ok(Self(std::fs::read_to_string(token_path()?)?))
    }
}
