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
        let body = serde_json::to_string(&body)?;
        tracing::info!("POST Body: {body}");
        let response = self
            .post(api_endpoint)
            .query(&query)
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
