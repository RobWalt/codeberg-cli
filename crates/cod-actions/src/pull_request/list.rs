use cod_cli::pull_request::list::ListPullRequestArgs;
use cod_client::CodebergClient;
use cod_render::spinner::spin_until_ready;

use cod_types::api::pull_request::PullRequest;

pub async fn list_pull(args: ListPullRequestArgs, client: &CodebergClient) -> anyhow::Result<()> {
    let pull_requests_list =
        spin_until_ready(client.get_repo_prs(Some(args.state), Some(args.count))).await?;

    present_pull_requests_list(pull_requests_list);

    Ok(())
}

fn present_pull_requests_list(pull_requests: Vec<PullRequest>) {
    use cod_render::prelude::*;

    let pull_requests_empty = pull_requests.is_empty();

    let rows = std::iter::once(Some(Row::new([TableCell::new_with_alignment(
        format!(
            "Pull Requests {}",
            pull_requests_empty
                .then_some(" (empty)")
                .unwrap_or_default()
        ),
        4,
        Alignment::Center,
    )])))
    .chain(std::iter::once_with(|| {
        (!pull_requests_empty).then(|| {
            Row::new([
                TableCell::new_with_alignment("Number", 1, Alignment::Center),
                TableCell::new_with_alignment("Status", 1, Alignment::Center),
                TableCell::new_with_alignment("Name", 1, Alignment::Center),
                TableCell::new_with_alignment("Labels", 1, Alignment::Center),
            ])
        })
    }))
    .chain(pull_requests.into_iter().map(|issue| {
        (!pull_requests_empty).then(|| {
            let PullRequest {
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
                    .map(|label| label.name)
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

    let table = CodTableBuilder::new().add_rows(rows).build();

    println!("{}", table.render());
}
