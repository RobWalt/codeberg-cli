use cod_cli::label::create::CreateLabelArgs;
use cod_render::spinner::spin_until_ready;
use cod_types::api::create_label_options::CreateLabelOption;
use cod_types::token::Token;
use reqwest::Url;

use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_types::api::label::Label;
use cod_types::client::CodebergClient;

pub async fn create_label(args: CreateLabelArgs, token: Token) -> anyhow::Result<()> {
    let client = CodebergClient::new(&token)?;

    let label = spin_until_ready(async {
        let api_endpoint = EndpointGenerator::repo_labels()?;

        create_label_post(&client, args, api_endpoint).await
    })
    .await?;

    println!("Successfully created label: {}", label.name);

    Ok(())
}

async fn create_label_post(
    client: &CodebergClient,
    args: CreateLabelArgs,
    api_endpoint: Url,
) -> anyhow::Result<Label> {
    let body = CreateLabelOption::new(args.name);
    let response = client.post_body(api_endpoint, body).await?;
    Ok(response)
}
