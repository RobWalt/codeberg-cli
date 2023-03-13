use crate::cli::label::delete::DeleteLabelArgs;
use crate::client::BergClient;
use crate::render::spinner::spin_until_ready;
use crate::render::ui::fuzzy_select_with_key;

use crate::actions::text_manipulation::select_prompt_for;

pub async fn delete_label(_args: DeleteLabelArgs, client: &BergClient) -> anyhow::Result<()> {
    let labels_list = spin_until_ready(client.get_repo_labels(None)).await?;

    let selected_label =
        fuzzy_select_with_key(labels_list, select_prompt_for("label")).and_then(|maybe_label| {
            maybe_label.ok_or_else(|| anyhow::anyhow!("No label for deletion selected, aborting."))
        })?;

    client.delete_label(selected_label.id).await?;

    println!("Successfully deleted label: {}", selected_label.name);

    Ok(())
}
