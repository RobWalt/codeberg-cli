use std::fmt::Debug;
use std::ops::Deref;
use std::ops::DerefMut;

use reqwest::header;
use reqwest::Client;
use reqwest::Url;
use serde::de::DeserializeOwned;

use crate::token::Token;

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

    pub async fn get<T: DeserializeOwned + Debug>(&self, api_endpoint: Url) -> anyhow::Result<T> {
        self.get_query::<[(&str, &str); 0], T>(api_endpoint, [])
            .await
    }

    pub async fn get_query<Q: serde::Serialize, T: DeserializeOwned + Debug>(
        &self,
        api_endpoint: Url,
        query: Q,
    ) -> anyhow::Result<T> {
        let request = self.deref().get(api_endpoint).query(&query);
        tracing::info!("Making GET call. Request: {:?}", request);
        let response = request.send().await?;
        tracing::info!("Response Status: {:?}", response.status());
        let json_response = response.json().await?;
        tracing::info!("Response: {:?}", json_response);
        Ok(json_response)
    }

    pub async fn post_body<B: serde::Serialize, T: DeserializeOwned + Debug>(
        &self,
        api_endpoint: Url,
        body: B,
    ) -> anyhow::Result<T> {
        tracing::info!(
            "Making POST call. API endpoint: {:?}",
            api_endpoint.as_str()
        );
        let body = serde_json::to_string(&body)?;
        tracing::info!("POST Body: {body}");
        let response = self
            .post(api_endpoint)
            .header(
                header::CONTENT_TYPE,
                "application/json".parse::<header::HeaderValue>()?,
            )
            .body(body)
            .send()
            .await?;
        tracing::info!("Response Status: {:?}", response.status());
        let json_response = response.json().await?;
        tracing::info!("Response: {:?}", json_response);
        Ok(json_response)
    }

    pub async fn patch_body<B: serde::Serialize, T: DeserializeOwned + Debug>(
        &self,
        api_endpoint: Url,
        body: B,
    ) -> anyhow::Result<T> {
        tracing::info!(
            "Making POST call. API endpoint: {:?}",
            api_endpoint.as_str()
        );
        let body = serde_json::to_string(&body)?;
        tracing::info!("POST Body: {body}");
        let response = self
            .patch(api_endpoint)
            .header(
                header::CONTENT_TYPE,
                "application/json".parse::<header::HeaderValue>()?,
            )
            .body(body)
            .send()
            .await?;
        tracing::info!("Response Status: {:?}", response.status());
        let json_response = response.json().await?;
        tracing::info!("Response: {:?}", json_response);
        Ok(json_response)
    }
}
