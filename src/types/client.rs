use std::ops::Deref;
use std::ops::DerefMut;

use reqwest::header;
use reqwest::Client;
use reqwest::Url;

use crate::Token;

#[derive(Debug, Clone)]
pub struct CodebergClient(Client);

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

impl CodebergClient {
    pub fn new(token: &Token) -> anyhow::Result<Self> {
        let Token(token) = token;
        let mut headers = header::HeaderMap::new();
        headers.insert(header::ACCEPT, "application/json".parse()?);
        headers.insert(header::AUTHORIZATION, format!("Bearer {token}").parse()?);

        let client = Client::builder().default_headers(headers).build()?;
        Ok(Self(client))
    }

    pub async fn get(&self, api_endpoint: Url) -> anyhow::Result<serde_json::Value> {
        let response = self.deref().get(api_endpoint).send().await?.json().await?;
        Ok(response)
    }
}
