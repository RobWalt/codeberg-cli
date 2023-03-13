use reqwest::Client;

#[derive(Debug, Clone)]
pub struct BergClient(pub(crate) Client);

use std::ops::Deref;
use std::ops::DerefMut;

impl Deref for BergClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BergClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
