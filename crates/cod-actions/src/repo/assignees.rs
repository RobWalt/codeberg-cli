use cod_cli::repo::assignees::RepoAssigneesArgs;
use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_render::spinner::spin_until_ready;
use cod_types::api::user::User;
use cod_types::client::CodebergClient;
use cod_types::token::Token;

pub async fn repo_assignees(args: RepoAssigneesArgs, token: Token) -> anyhow::Result<()> {
    let client = CodebergClient::new(&token)?;

    let repo_assignees = spin_until_ready(get_assignees_data(&client)).await?;

    present_repo_assignees(repo_assignees);

    Ok(())
}

async fn get_assignees_data(client: &CodebergClient) -> anyhow::Result<Vec<User>> {
    let api_endpoint = EndpointGenerator::repo_assignees()?;

    let repo_assignees = client.get(api_endpoint).await?;
    Ok(repo_assignees)
}

fn present_repo_assignees(repo_assignees: Vec<User>) {
    use cod_render::prelude::*;

    let rows = std::iter::once(Row::new([TableCell::new_with_alignment(
        format!(
            "Repository Assignees{}",
            repo_assignees
                .is_empty()
                .then_some(" (empty)")
                .unwrap_or_default()
        ),
        1,
        Alignment::Left,
    )]))
    .chain(repo_assignees.into_iter().map(|assignee| {
        Row::new([TableCell::new_with_alignment(
            assignee.username,
            1,
            Alignment::Center,
        )])
    }))
    .collect::<Vec<_>>();

    let table = CodTableBuilder::new().add_rows(rows).build();

    println!("{}", table.render());
}
