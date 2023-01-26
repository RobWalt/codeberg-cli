use std::ops::Deref;

use reqwest::Url;

use crate::CodebergClient;

impl CodebergClient {
    pub async fn delete(&self, api_endpoint: Url) -> anyhow::Result<()> {
        let request = self.deref().delete(api_endpoint);
        tracing::info!("Making DELETE call. Request: {request:?}");
        let response = request.send().await?;
        let status = response.status();
        tracing::info!("Response status: {status:?}");
        if !status.is_success() {
            anyhow::bail!("Deleting failed: {}", response.text().await?);
        }
        Ok(())
    }
}
