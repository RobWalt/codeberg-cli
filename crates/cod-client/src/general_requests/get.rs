use reqwest::Url;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::ops::Deref;

use crate::CodebergClient;

impl CodebergClient {
    pub async fn get<T>(&self, api_endpoint: Url) -> anyhow::Result<T>
    where
        T: DeserializeOwned + Debug,
    {
        self.get_query::<[(); 0], T>(api_endpoint, []).await
    }

    pub async fn get_query<Q, T>(&self, api_endpoint: Url, query: Q) -> anyhow::Result<T>
    where
        Q: Serialize + Debug,
        T: DeserializeOwned + Debug,
    {
        tracing::debug!("Query: {query:?}");
        let request = self.deref().get(api_endpoint.clone()).query(&query);
        tracing::debug!("Making GET call. Request: {:?}", request);
        let response = request.send().await?;
        tracing::debug!("Response Status: {:?}", response.status());
        let json_response = response.json().await;
        if json_response.is_err() {
            let request = self.deref().get(api_endpoint).query(&query);
            let response = request.send().await?;
            tracing::debug!("======\n{}\n======", response.text().await?);
        }
        let json_response = json_response?;
        tracing::debug!("Response: {:?}", json_response);
        Ok(json_response)
    }
}
