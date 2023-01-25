use reqwest::Client;

#[derive(Debug, Clone)]
pub struct CodebergClient(pub(crate) Client);

use std::ops::Deref;
use std::ops::DerefMut;

impl Deref for CodebergClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CodebergClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
