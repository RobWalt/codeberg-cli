use cod_cli::label::list::ListLabelsArgs;
use cod_types::token::Token;
use reqwest::Url;

use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_git_info::reponame::get_reponame;
use cod_git_info::username::get_username;
use cod_types::api::label::Label;
use cod_types::client::CodebergClient;

pub async fn list_labels(args: ListLabelsArgs, token: Token) -> anyhow::Result<()> {
    let client = CodebergClient::new(&token)?;
    let repo_name = get_reponame()?;
    let username = get_username(&client).await?;

    let api_endpoint = EndpointGenerator::list_labels(username, repo_name)?;

    let labels_list = get_labels_list(&client, args, api_endpoint).await?;

    present_labels_list(labels_list);

    Ok(())
}

async fn get_labels_list(
    client: &CodebergClient,
    args: ListLabelsArgs,
    api_endpoint: Url,
) -> anyhow::Result<Vec<Label>> {
    client
        .get_query::<_, Vec<Label>>(api_endpoint, [("limit", args.count)])
        .await
}

fn present_labels_list(labels: Vec<Label>) {
    use cod_render::prelude::*;

    let labels_empty = labels.is_empty();

    let rows = std::iter::once(Row::new([TableCell::new_with_alignment(
        format!(
            "Labels{}",
            labels_empty.then_some(" (empty)").unwrap_or_default()
        ),
        1,
        Alignment::Center,
    )]))
    .chain(labels.into_iter().map(|label| {
        Row::new([TableCell::new_with_alignment(
            label.name,
            1,
            Alignment::Left,
        )])
    }));

    let table = CodTableBuilder::new()
        .with_max_column_width(100)
        .add_rows(rows)
        .build();

    println!("{}", table.render());
}