use crate::cli::issue::list::ListIssueArgs;
use crate::client::BergClient;
use crate::render::spinner::spin_until_ready;
use crate::types::api::issue::Issue;

pub async fn list_issue(args: ListIssueArgs, client: &BergClient) -> anyhow::Result<()> {
    let mut issues_list = spin_until_ready(client.get_repo_issues(None, Some(args.count))).await?;

    issues_list.retain(|issue| issue.pull_request.is_none());

    present_issues_list(issues_list);

    Ok(())
}

fn present_issues_list(issues: Vec<Issue>) {
    use crate::render::table::builder::BergTableBuilder;
    use term_table::row::Row;
    use term_table::table_cell::{Alignment, TableCell};

    let issues_empty = issues.is_empty();

    let rows = std::iter::once(Some(Row::new([TableCell::new_with_alignment(
        format!(
            "Issues{}",
            issues_empty.then_some(" (empty)").unwrap_or_default()
        ),
        4,
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
                ..
            } = issue;
            let labels = if labels.is_empty() {
                String::from("x")
            } else {
                labels
                    .into_iter()
                    .map(|label| format!("- {}", label.name))
                    .collect::<Vec<_>>()
                    .join("\n")
            };
            Row::new([
                TableCell::new_with_alignment(number, 1, Alignment::Left),
                TableCell::new_with_alignment(state, 1, Alignment::Left),
                TableCell::new_with_alignment(title, 1, Alignment::Left),
                TableCell::new_with_alignment(labels, 1, Alignment::Left),
            ])
        })
    }))
    .flatten();

    let table = BergTableBuilder::new().add_rows(rows).build();

    println!("{}", table.render());
}
