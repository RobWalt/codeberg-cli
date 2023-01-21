use anyhow::Context;

use crate::endpoints::endpoint_generator::EndpointGenerator;
use crate::types::client::CodebergClient;

pub async fn get_username(client: &CodebergClient) -> anyhow::Result<String> {
    let api_endpoint = EndpointGenerator::user_info()?;
    client
        .get::<serde_json::Value>(api_endpoint)
        .await?
        .get("username")
        .and_then(|username| username.as_str().map(|name| name.to_owned()))
        .context("Couldn't find username in Response.")
}
