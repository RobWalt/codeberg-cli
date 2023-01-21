use cod_cli::pull_request::list::ListPullRequestArgs;
use cod_types::token::Token;
use reqwest::Url;

use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_git_info::reponame::get_reponame;
use cod_git_info::username::get_username;
use cod_types::api::pull_request::PullRequest;
use cod_types::client::CodebergClient;

pub async fn list_pull(args: ListPullRequestArgs, token: Token) -> anyhow::Result<()> {
    let client = CodebergClient::new(&token)?;
    let repo_name = get_reponame()?;
    let username = get_username(&client).await?;

    let api_endpoint = EndpointGenerator::pull_list(username, repo_name)?;

    let pull_requests_list = get_pull_list(&client, args, api_endpoint).await?;

    present_pull_requests_list(pull_requests_list);

    Ok(())
}

async fn get_pull_list(
    client: &CodebergClient,
    args: ListPullRequestArgs,
    api_endpoint: Url,
) -> anyhow::Result<Vec<PullRequest>> {
    client
        .get_query::<_, Vec<PullRequest>>(api_endpoint, [("limit", args.count)])
        .await
}

fn present_pull_requests_list(pull_requests: Vec<PullRequest>) {
    use term_table::row::*;
    use term_table::table_cell::*;
    use term_table::*;

    let mut table = Table::new();
    table.max_column_width(40);
    table.style = TableStyle::elegant();

    let pull_requests_empty = pull_requests.is_empty();

    let header = if pull_requests_empty {
        "Pull Requests (empty)"
    } else {
        "Pull Requests"
    };

    table.add_row(Row::new(vec![TableCell::new_with_alignment(
        header,
        3,
        Alignment::Center,
    )]));

    if !pull_requests_empty {
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Number", 1, Alignment::Center),
            TableCell::new_with_alignment("Name", 1, Alignment::Center),
            TableCell::new_with_alignment("Labels", 1, Alignment::Center),
        ]));

        pull_requests.into_iter().for_each(|pull_request| {
            let PullRequest {
                title,
                number,
                labels,
            } = pull_request;
            table.add_row(Row::new(vec![
                TableCell::new_with_alignment(number, 1, Alignment::Left),
                TableCell::new_with_alignment(title, 1, Alignment::Left),
                TableCell::new_with_alignment(
                    labels
                        .into_iter()
                        .map(|label| label.name)
                        .collect::<Vec<_>>()
                        .join("\n"),
                    1,
                    Alignment::Left,
                ),
            ]));
        });
    }

    println!("{}", table.render());
}
