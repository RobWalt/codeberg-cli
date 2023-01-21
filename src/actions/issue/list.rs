use reqwest::Url;

use crate::endpoints::endpoint_generator::EndpointGenerator;
use crate::git_info::reponame::get_reponame;
use crate::git_info::username::get_username;
use crate::types::api::issue::Issue;
use crate::types::client::CodebergClient;
use crate::{ListArgs, Token};

pub async fn list(args: ListArgs, token: Token) -> anyhow::Result<()> {
    let client = CodebergClient::new(&token)?;
    let repo_name = get_reponame()?;
    let username = get_username(&client).await?;

    let api_endpoint = EndpointGenerator::issues_list(username, repo_name)?;

    let issues_list = get_issue_list(&client, args, api_endpoint).await?;

    present_issues_list(issues_list);

    Ok(())
}

async fn get_issue_list(
    client: &CodebergClient,
    args: ListArgs,
    api_endpoint: Url,
) -> anyhow::Result<Vec<Issue>> {
    client
        .get_query::<_, Vec<Issue>>(api_endpoint, [("limit", args.count)])
        .await
}

fn present_issues_list(issues: Vec<Issue>) {
    use term_table::row::*;
    use term_table::table_cell::*;
    use term_table::*;

    let mut table = Table::new();
    table.max_column_width(40);
    table.style = TableStyle::elegant();

    let issues_empty = issues.is_empty();

    let header = if issues_empty {
        "Issues (empty)"
    } else {
        "Issues"
    };

    table.add_row(Row::new(vec![TableCell::new_with_alignment(
        header,
        3,
        Alignment::Center,
    )]));

    if !issues_empty {
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Number", 1, Alignment::Center),
            TableCell::new_with_alignment("Name", 1, Alignment::Center),
            TableCell::new_with_alignment("Labels", 1, Alignment::Center),
        ]));

        issues.into_iter().for_each(|issue| {
            let Issue {
                title,
                number,
                labels,
            } = issue;
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
