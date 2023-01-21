use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_types::api::user_info::UserInfo;
use cod_types::client::CodebergClient;

pub async fn get_username(client: &CodebergClient) -> anyhow::Result<String> {
    let api_endpoint = EndpointGenerator::user_info()?;
    let user_info = client.get::<UserInfo>(api_endpoint).await?;
    Ok(user_info.username)
}
