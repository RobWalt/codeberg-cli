use crate::cli::pull_request::view::ViewPullRequestsArgs;
use crate::client::BergClient;
use crate::render::comment::render_comment;
use crate::render::datetime::render_datetime_and_info;
use crate::render::spinner::spin_until_ready;
use crate::render::table::builder::BergTableBuilder;
use crate::render::ui::fuzzy_select_with_key;
use crate::types::api::pull_request::PullRequest;
use std::iter::once;
use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};

use crate::actions::text_manipulation::select_prompt_for;

pub async fn view_pull(args: ViewPullRequestsArgs, client: &BergClient) -> anyhow::Result<()> {
    let pull_requests_list = spin_until_ready(client.get_repo_prs(Some(args.state), None)).await?;

    let selected_pull_request =
        fuzzy_select_with_key(pull_requests_list, select_prompt_for("pull request"))?;

    if args.comments {
        spin_until_ready(present_pull_request_comments(client, selected_pull_request)).await?;
    } else {
        present_pull_request_overview(selected_pull_request);
    }

    Ok(())
}

fn present_pull_request_overview(selected_pull_request: Option<PullRequest>) {
    let rendered_datetime = selected_pull_request
        .as_ref()
        .map(|pull_request| render_datetime_and_info(pull_request.created_at));

    let rows = once(Some(Row::new([TableCell::new_with_alignment(
        selected_pull_request
            .as_ref()
            .map(|pull_request| format!("Pull Request #{}", pull_request.number))
            .unwrap_or_else(|| String::from("No Pull Requests available")),
        2,
        Alignment::Center,
    )])))
    .chain(once(selected_pull_request.as_ref().map(|pull_request| {
        Row::new([
            TableCell::new_with_alignment("Title", 1, Alignment::Center),
            TableCell::new_with_alignment(pull_request.title.as_str(), 1, Alignment::Left),
        ])
    })))
    .chain(rendered_datetime.into_iter().map(|creation_time| {
        Some(Row::new([
            TableCell::new_with_alignment("Created", 1, Alignment::Center),
            TableCell::new_with_alignment(creation_time, 1, Alignment::Left),
        ]))
    }))
    .chain(once(selected_pull_request.as_ref().map(|pull_request| {
        Row::new([
            TableCell::new_with_alignment("Labels", 1, Alignment::Center),
            TableCell::new_with_alignment(
                pull_request
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
    .chain(once(selected_pull_request.as_ref().map(|pull_request| {
        Row::new([
            TableCell::new_with_alignment("Description", 1, Alignment::Center),
            TableCell::new_with_alignment(pull_request.body.as_str(), 1, Alignment::Left),
        ])
    })))
    .flatten();

    let table = BergTableBuilder::new().add_rows(rows).build();

    println!("{}", table.render());
}

async fn present_pull_request_comments(
    client: &BergClient,
    selected_pull_request: Option<PullRequest>,
) -> anyhow::Result<()> {
    let (header, comments) = if let Some(pull_request) = selected_pull_request.as_ref() {
        let comments_list = client.get_comments_for_id(pull_request.number).await?;
        let header = format!(
            "Pull Request #{} {}",
            pull_request.number,
            if comments_list.is_empty() {
                "(no comments)"
            } else {
                "comments"
            }
        );
        (header, comments_list)
    } else {
        (String::from("No Pull Requests available"), vec![])
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
