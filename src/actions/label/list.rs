use crate::cli::label::list::ListLabelsArgs;
use crate::client::BergClient;
use crate::render::spinner::spin_until_ready;

use crate::types::api::label::Label;

pub async fn list_label(args: ListLabelsArgs, client: &BergClient) -> anyhow::Result<()> {
    let labels_list = spin_until_ready(client.get_repo_labels(Some(args.count))).await?;

    present_labels_list(labels_list);

    Ok(())
}

fn present_labels_list(labels: Vec<Label>) {
    use crate::render::table::builder::BergTableBuilder;
    use term_table::row::Row;
    use term_table::table_cell::{Alignment, TableCell};

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

    let table = BergTableBuilder::new()
        .with_max_column_width(100)
        .add_rows(rows)
        .build();

    println!("{}", table.render());
}
