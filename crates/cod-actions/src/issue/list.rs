use cod_cli::issue::list::ListIssueArgs;
use cod_client::CodebergClient;
use cod_render::spinner::spin_until_ready;

use cod_types::api::issue::Issue;

pub async fn list_issues(args: ListIssueArgs, client: &CodebergClient) -> anyhow::Result<()> {
    let issues_list = spin_until_ready(client.get_repo_issues(None, Some(args.count))).await?;

    present_issues_list(issues_list);

    Ok(())
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
                TableCell::new_with_alignment("Status", 1, Alignment::Center),
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
                state,
                assignees: _assignees,
                body: _body,
            } = issue;
            Row::new([
                TableCell::new_with_alignment(number, 1, Alignment::Left),
                TableCell::new_with_alignment(state, 1, Alignment::Left),
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
