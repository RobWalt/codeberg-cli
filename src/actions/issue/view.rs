use crate::cli::issue::view::ViewIssueArgs;
use crate::client::BergClient;
use crate::render::comment::render_comment;
use crate::render::datetime::render_datetime_and_info;
use crate::render::spinner::spin_until_ready;
use crate::render::table::builder::BergTableBuilder;
use crate::render::ui::fuzzy_select_with_key;
use crate::types::api::issue::Issue;
use std::iter::once;
use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};

use crate::actions::text_manipulation::select_prompt_for;

pub async fn view_issue(args: ViewIssueArgs, client: &BergClient) -> anyhow::Result<()> {
    let issues_list = spin_until_ready(client.get_repo_issues(Some(args.state), None)).await?;

    let selected_issue = fuzzy_select_with_key(issues_list, select_prompt_for("issue"))?;

    if args.comments {
        spin_until_ready(present_issue_comments(client, selected_issue)).await?;
    } else {
        present_issue_overview(selected_issue);
    }

    Ok(())
}

fn present_issue_overview(selected_issue: Option<Issue>) {
    let days_passed_since_creation = selected_issue
        .as_ref()
        .map(|issue| render_datetime_and_info(issue.created_at));

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
    .chain(days_passed_since_creation.into_iter().map(|creation_time| {
        Some(Row::new([
            TableCell::new_with_alignment("Created", 1, Alignment::Center),
            TableCell::new_with_alignment(creation_time, 1, Alignment::Left),
        ]))
    }))
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

    let table = BergTableBuilder::new().add_rows(rows).build();

    println!("{}", table.render());
}

async fn present_issue_comments(
    client: &BergClient,
    selected_issue: Option<Issue>,
) -> anyhow::Result<()> {
    let (header, comments) = if let Some(issue) = selected_issue.as_ref() {
        let comments_list = client.get_comments_for_id(issue.number).await?;
        let header = format!(
            "Issue #{} {}",
            issue.number,
            if comments_list.is_empty() {
                "(no comments)"
            } else {
                "comments"
            }
        );
        (header, comments_list)
    } else {
        (String::from("No Issues available"), vec![])
    };

    let rows = once(Row::new([TableCell::new_with_alignment(
        header,
        1,
        Alignment::Center,
    )]))
    .chain(comments.into_iter().map(|comment| {
        tracing::debug!("comment:{comment:?}");
        Row::new([TableCell::new_with_alignment(
            render_comment(
                comment.user.username.as_str(),
                comment.created_at,
                comment.body.as_str(),
                50,
            ),
            1,
            Alignment::Left,
        )])
    }));

    let table = BergTableBuilder::new()
        .add_rows(rows)
        .with_max_column_width(50)
        .build();

    println!("{}", table.render());

    Ok(())
}
