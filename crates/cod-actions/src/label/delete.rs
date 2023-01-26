use cod_cli::label::delete::DeleteLabelArgs;
use cod_client::CodebergClient;
use cod_render::spinner::spin_until_ready;
use cod_render::ui::fuzzy_select_with_key;

pub async fn delete_label(_args: DeleteLabelArgs, client: &CodebergClient) -> anyhow::Result<()> {
    let labels_list = spin_until_ready(client.get_repo_labels(None)).await?;

    let selected_label =
        fuzzy_select_with_key(labels_list, |label| label.name.to_owned(), |label| label).and_then(
            |maybe_label| {
                maybe_label
                    .ok_or_else(|| anyhow::anyhow!("No label for deletion selected, aborting."))
            },
        )?;

    client.delete_label(selected_label.id).await?;

    println!("Successfully deleted label: {}", selected_label.name);

    Ok(())
}
