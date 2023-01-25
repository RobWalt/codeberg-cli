use reqwest::{header, Url};
use serde::de::DeserializeOwned;
use std::fmt::Debug;

use crate::CodebergClient;

impl CodebergClient {
    pub async fn patch_body<B, T>(&self, api_endpoint: Url, body: B) -> anyhow::Result<T>
    where
        B: serde::Serialize,
        T: DeserializeOwned + Debug,
    {
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
