use reqwest::{header, Client};

use crate::types::token::Token;

use crate::client::BergClient;

impl BergClient {
    pub fn new(token: &Token) -> anyhow::Result<Self> {
        let Token(token) = token;
        let mut headers = header::HeaderMap::new();
        headers.insert(header::ACCEPT, "application/json".parse()?);
        headers.insert(header::AUTHORIZATION, format!("Bearer {token}").parse()?);

        let client = Client::builder().default_headers(headers).build()?;
        Ok(Self(client))
    }
}
