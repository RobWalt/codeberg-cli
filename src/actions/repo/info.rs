use crate::cli::repo::info::RepoInfoArgs;
use crate::client::BergClient;
use crate::render::spinner::spin_until_ready;
use crate::types::api::repository::Repository;

pub async fn info_repo(_args: RepoInfoArgs, client: &BergClient) -> anyhow::Result<()> {
    let repo_data = spin_until_ready(client.get_repo_info()).await?;

    present_repo_info(repo_data);

    Ok(())
}

fn present_repo_info(repo_data: Repository) {
    use crate::render::table::builder::BergTableBuilder;
    use term_table::row::Row;
    use term_table::table_cell::{Alignment, TableCell};

    let rows = [
        Row::new([
            TableCell::new_with_alignment("Repository Name", 1, Alignment::Left),
            TableCell::new_with_alignment(repo_data.name, 1, Alignment::Center),
        ]),
        Row::new([
            TableCell::new_with_alignment("Repository Owner", 1, Alignment::Left),
            TableCell::new_with_alignment(repo_data.owner.username, 1, Alignment::Center),
        ]),
        Row::new([
            TableCell::new_with_alignment("Stars", 1, Alignment::Left),
            TableCell::new_with_alignment(
                format!("{}⭐", repo_data.stars_count),
                1,
                Alignment::Center,
            ),
        ]),
    ];

    let table = BergTableBuilder::new().add_rows(rows).build();

    println!("{}", table.render());
}
