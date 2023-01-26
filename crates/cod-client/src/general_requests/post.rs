use std::fmt::Debug;

use reqwest::header;
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::CodebergClient;

impl CodebergClient {
    pub async fn post_body<B: serde::Serialize, T: DeserializeOwned + Debug>(
        &self,
        api_endpoint: Url,
        body: B,
    ) -> anyhow::Result<T> {
        self.post_query_body::<[(); 0], B, T>(api_endpoint, body, [])
            .await
    }

    pub async fn post_query_body<Q: Serialize, B: Serialize, T: DeserializeOwned + Debug>(
        &self,
        api_endpoint: Url,
        body: B,
        query: Q,
    ) -> anyhow::Result<T> {
        tracing::info!(
            "Making POST call. API endpoint: {:?}",
            api_endpoint.as_str()
        );
        let body_str = serde_json::to_string(&body)?;
        tracing::info!("POST Body: {body_str}");
        let response = self
            .post(api_endpoint.clone())
            .query(&query)
            .header(
                header::CONTENT_TYPE,
                "application/json".parse::<header::HeaderValue>()?,
            )
            .body(body_str)
            .send()
            .await?;
        let status = response.status();
        tracing::info!("Response Status: {status:?}");
        if !status.is_success() {
            let body_str = serde_json::to_string(&body)?;
            let response = self
                .post(api_endpoint)
                .query(&query)
                .header(
                    header::CONTENT_TYPE,
                    "application/json".parse::<header::HeaderValue>()?,
                )
                .body(body_str)
                .send()
                .await?;
            let text = response.text().await?;
            tracing::info!("Failed POST: {text}");
        }
        let json_response = response.json().await?;
        tracing::info!("Response: {:?}", json_response);
        Ok(json_response)
    }
}
