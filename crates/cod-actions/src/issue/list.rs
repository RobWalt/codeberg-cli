use cod_cli::issue::list::ListIssueArgs;
use cod_render::spinner::spin_until_ready;
use cod_types::token::Token;
use reqwest::Url;

use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_git_info::reponame::get_reponame;
use cod_git_info::username::get_username;
use cod_types::api::issue::Issue;
use cod_types::client::CodebergClient;

pub async fn list_issues(args: ListIssueArgs, token: Token) -> anyhow::Result<()> {
    let client = CodebergClient::new(&token)?;

    let issues_list = spin_until_ready(async {
        let repo_name = get_reponame()?;
        let username = get_username(&client).await?;

        let api_endpoint = EndpointGenerator::repo_issues(username, repo_name)?;

        get_issue_list(&client, args, api_endpoint).await
    })
    .await?;

    present_issues_list(issues_list);

    Ok(())
}

async fn get_issue_list(
    client: &CodebergClient,
    args: ListIssueArgs,
    api_endpoint: Url,
) -> anyhow::Result<Vec<Issue>> {
    client
        .get_query::<_, Vec<Issue>>(api_endpoint, [("limit", args.count)])
        .await
}

fn present_issues_list(issues: Vec<Issue>) {
    use cod_render::prelude::*;

    let issues_empty = issues.is_empty();

    let rows = std::iter::once(Some(Row::new([TableCell::new_with_alignment(
        format!(
            "Issues{}",
            issues_empty.then_some(" (empty)").unwrap_or_default()
        ),
        3,
        Alignment::Center,
    )])))
    .chain(std::iter::once_with(|| {
        (!issues_empty).then(|| {
            Row::new([
                TableCell::new_with_alignment("Number", 1, Alignment::Center),
                TableCell::new_with_alignment("Name", 1, Alignment::Center),
                TableCell::new_with_alignment("Labels", 1, Alignment::Center),
            ])
        })
    }))
    .chain(issues.into_iter().map(|issue| {
        (!issues_empty).then(|| {
            let Issue {
                title,
                number,
                labels,
            } = issue;
            Row::new([
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
            ])
        })
    }))
    .flatten();

    let table = CodTableBuilder::new().add_rows(rows).build();

    println!("{}", table.render());
}
