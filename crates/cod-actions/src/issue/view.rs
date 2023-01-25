use cod_cli::issue::view::ViewIssueArgs;
use cod_client::CodebergClient;
use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_render::spinner::spin_until_ready;
use cod_render::ui::fuzzy_select_with_key;
use cod_types::api::issue::Issue;

pub async fn view_issue(args: ViewIssueArgs, client: &CodebergClient) -> anyhow::Result<()> {
    let issues_list = spin_until_ready(get_issues_list(client, args)).await?;

    let selected_issue = fuzzy_select_with_key(
        issues_list,
        |issue: &Issue| format!("#{} {}", issue.number, issue.title),
        |issue| issue,
    )?;

    present_selected_issue(selected_issue);

    Ok(())
}

async fn get_issues_list(
    client: &CodebergClient,
    args: ViewIssueArgs,
) -> anyhow::Result<Vec<Issue>> {
    let api_endpoint = EndpointGenerator::repo_issues()?;
    client
        .get_query::<_, Vec<Issue>>(api_endpoint, [("state", args.state.to_string().as_str())])
        .await
}

fn present_selected_issue(selected_issue: Option<Issue>) {
    use cod_render::prelude::*;
    use std::iter::once;

    let rows = once(Some(Row::new([TableCell::new_with_alignment(
        selected_issue
            .as_ref()
            .map(|issue| format!("Issue #{}", issue.number))
            .unwrap_or_else(|| String::from("No Issues available")),
        2,
        Alignment::Center,
    )])))
    .chain(once(selected_issue.as_ref().map(|issue| {
        Row::new([
            TableCell::new_with_alignment("Title", 1, Alignment::Center),
            TableCell::new_with_alignment(issue.title.as_str(), 1, Alignment::Left),
        ])
    })))
    .chain(once(selected_issue.as_ref().map(|issue| {
        Row::new([
            TableCell::new_with_alignment("Labels", 1, Alignment::Center),
            TableCell::new_with_alignment(
                issue
                    .labels
                    .iter()
                    .map(|label| label.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
                1,
                Alignment::Left,
            ),
        ])
    })))
    .chain(once(selected_issue.as_ref().map(|issue| {
        Row::new([
            TableCell::new_with_alignment("Description", 1, Alignment::Center),
            TableCell::new_with_alignment(issue.body.as_str(), 1, Alignment::Left),
        ])
    })))
    .flatten();

    let table = CodTableBuilder::new().add_rows(rows).build();

    println!("{}", table.render());
}
