use cod_cli::label::list::ListLabelsArgs;
use cod_render::spinner::spin_until_ready;
use reqwest::Url;

use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_types::api::label::Label;
use cod_types::client::CodebergClient;

pub async fn list_labels(args: ListLabelsArgs, client: &CodebergClient) -> anyhow::Result<()> {
    let labels_list = spin_until_ready(async {
        let api_endpoint = EndpointGenerator::repo_labels()?;

        get_labels_list(client, args, api_endpoint).await
    })
    .await?;

    present_labels_list(labels_list);

    Ok(())
}

async fn get_labels_list(
    client: &CodebergClient,
    args: ListLabelsArgs,
    api_endpoint: Url,
) -> anyhow::Result<Vec<Label>> {
    client
        .get_query::<_, Vec<Label>>(api_endpoint, [("limit", args.count)])
        .await
}

fn present_labels_list(labels: Vec<Label>) {
    use cod_render::prelude::*;

    let labels_empty = labels.is_empty();

    let rows = std::iter::once(Row::new([TableCell::new_with_alignment(
        format!(
            "Labels{}",
            labels_empty.then_some(" (empty)").unwrap_or_default()
        ),
        1,
        Alignment::Center,
    )]))
    .chain(labels.into_iter().map(|label| {
        Row::new([TableCell::new_with_alignment(
            label.name,
            1,
            Alignment::Left,
        )])
    }));

    let table = CodTableBuilder::new()
        .with_max_column_width(100)
        .add_rows(rows)
        .build();

    println!("{}", table.render());
}
