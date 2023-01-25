use cod_cli::label::list::ListLabelsArgs;
use cod_client::CodebergClient;
use cod_render::spinner::spin_until_ready;

use cod_types::api::label::Label;

pub async fn list_labels(args: ListLabelsArgs, client: &CodebergClient) -> anyhow::Result<()> {
    let labels_list = spin_until_ready(client.get_repo_labels(Some(args.count))).await?;

    present_labels_list(labels_list);

    Ok(())
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
