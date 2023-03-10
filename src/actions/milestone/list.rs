use crate::cli::milestone::list::ListMilestonesArgs;
use crate::client::BergClient;
use crate::render::spinner::spin_until_ready;
use crate::types::api::milestone::Milestone;

pub async fn list_milestone(args: ListMilestonesArgs, client: &BergClient) -> anyhow::Result<()> {
    let milestones_list =
        spin_until_ready(client.get_repo_milestones(None, Some(args.count))).await?;

    present_milestones_list(milestones_list);

    Ok(())
}

fn present_milestones_list(milestones: Vec<Milestone>) {
    use crate::render::table::builder::BergTableBuilder;
    use term_table::row::Row;
    use term_table::table_cell::{Alignment, TableCell};

    let milestones_empty = milestones.is_empty();

    let rows = std::iter::once(Row::new([TableCell::new_with_alignment(
        format!(
            "Milestones{}",
            milestones_empty.then_some(" (empty)").unwrap_or_default()
        ),
        1,
        Alignment::Center,
    )]))
    .chain(milestones.into_iter().map(|milestone| {
        Row::new([TableCell::new_with_alignment(
            milestone.title,
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
