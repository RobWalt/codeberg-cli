use cod_cli::pull_request::list::ListPullRequestArgs;
use cod_render::spinner::spin_until_ready;
use reqwest::Url;

use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_types::api::pull_request::PullRequest;
use cod_types::client::CodebergClient;

pub async fn list_pulls(args: ListPullRequestArgs, client: &CodebergClient) -> anyhow::Result<()> {
    let pull_requests_list = spin_until_ready(async {
        let api_endpoint = EndpointGenerator::repo_pulls()?;

        get_pull_list(client, args, api_endpoint).await
    })
    .await?;

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
    use cod_render::prelude::*;

    let pull_requests_empty = pull_requests.is_empty();

    let rows = std::iter::once(Some(Row::new([TableCell::new_with_alignment(
        format!(
            "Issues{}",
            pull_requests_empty
                .then_some(" (empty)")
                .unwrap_or_default()
        ),
        3,
        Alignment::Center,
    )])))
    .chain(std::iter::once_with(|| {
        (!pull_requests_empty).then(|| {
            Row::new([
                TableCell::new_with_alignment("Number", 1, Alignment::Center),
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
