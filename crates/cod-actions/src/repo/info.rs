use cod_cli::repo::info::RepoInfoArgs;
use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_render::spinner::spin_until_ready;
use cod_types::api::repository::Repository;
use cod_types::client::CodebergClient;
use cod_types::token::Token;

pub async fn repo_info(_args: RepoInfoArgs, token: Token) -> anyhow::Result<()> {
    let client = CodebergClient::new(&token)?;

    let repo_data = spin_until_ready(get_user_data(&client)).await?;

    present_repo_info(repo_data);

    Ok(())
}

async fn get_user_data(client: &CodebergClient) -> anyhow::Result<Repository> {
    let api_endpoint = EndpointGenerator::repo_infos()?;
    let repo_info = client.get(api_endpoint).await?;
    Ok(repo_info)
}

fn present_repo_info(repo_data: Repository) {
    use cod_render::prelude::*;

    let rows = [
        Row::new([
            TableCell::new_with_alignment("Repository Name", 1, Alignment::Left),
            TableCell::new_with_alignment(repo_data.name, 1, Alignment::Center),
        ]),
        Row::new([
            TableCell::new_with_alignment("Repository Owner", 1, Alignment::Left),
            TableCell::new_with_alignment(repo_data.owner.username, 1, Alignment::Center),
        ]),
        Row::new([
            TableCell::new_with_alignment("Stars", 1, Alignment::Left),
            TableCell::new_with_alignment(
                format!("{}⭐", repo_data.stars_count),
                1,
                Alignment::Center,
            ),
        ]),
    ];

    let table = CodTableBuilder::new().add_rows(rows).build();

    println!("{}", table.render());
}
