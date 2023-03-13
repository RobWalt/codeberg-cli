use std::fmt::Debug;

use reqwest::{header, Url};
use serde::de::DeserializeOwned;

use crate::client::BergClient;

impl BergClient {
    pub async fn put_body<B, T>(&self, api_endpoint: Url, body: B) -> anyhow::Result<T>
    where
        B: serde::Serialize,
        T: DeserializeOwned + Debug,
    {
        tracing::debug!("Making PUT call. API endpoint: {:?}", api_endpoint.as_str());
        let body = serde_json::to_string(&body)?;
        tracing::debug!("PUT Body: {body}");
        let response = self
            .put(api_endpoint.clone())
            .header(
                header::CONTENT_TYPE,
                "application/json".parse::<header::HeaderValue>()?,
            )
            .body(body.clone())
            .send()
            .await?;
        tracing::debug!("Response Status: {:?}", response.status());
        if !response.status().is_success() {
            let response = self
                .put(api_endpoint.clone())
                .header(
                    header::CONTENT_TYPE,
                    "application/json".parse::<header::HeaderValue>()?,
                )
                .body(body)
                .send()
                .await?;
            let text = response.text().await?;
            tracing::debug!("{text}");
        }
        let json_response = response.json().await?;
        tracing::debug!("Response: {:?}", json_response);
        Ok(json_response)
    }
}
