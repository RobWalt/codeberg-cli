use crate::cli::repo::assignees::RepoAssigneesArgs;
use crate::client::BergClient;
use crate::endpoints::endpoint_generator::EndpointGenerator;
use crate::render::spinner::spin_until_ready;
use crate::types::api::user::User;

pub async fn assignees_repo(_args: RepoAssigneesArgs, client: &BergClient) -> anyhow::Result<()> {
    let repo_assignees = spin_until_ready(get_assignees_data(client)).await?;

    present_repo_assignees(repo_assignees);

    Ok(())
}

async fn get_assignees_data(client: &BergClient) -> anyhow::Result<Vec<User>> {
    let api_endpoint = EndpointGenerator::repo_assignees()?;

    let repo_assignees = client.get(api_endpoint).await?;
    Ok(repo_assignees)
}

fn present_repo_assignees(repo_assignees: Vec<User>) {
    use crate::render::table::builder::BergTableBuilder;
    use term_table::row::Row;
    use term_table::table_cell::{Alignment, TableCell};

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

    let table = BergTableBuilder::new().add_rows(rows).build();

    println!("{}", table.render());
}
