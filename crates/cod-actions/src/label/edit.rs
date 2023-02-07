use cod_cli::label::edit::EditLabelArgs;
use cod_client::CodebergClient;
use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_render::spinner::spin_until_ready;
use cod_render::ui::{fuzzy_select_with_key, multi_fuzzy_select_with_key};
use cod_types::api::edit_options::edit_label_option::EditLabelOption;
use cod_types::api::label::Label;
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::text_manipulation::select_prompt_for;

#[derive(Display, EnumIter, PartialEq, Eq)]
enum EditableFields {
    Name,
    Description,
    Color,
}

pub async fn edit_label(_args: EditLabelArgs, client: &CodebergClient) -> anyhow::Result<()> {
    let labels_list = spin_until_ready(client.get_repo_labels(None)).await?;

    if labels_list.is_empty() {
        println!("No labels found in this repository");
    }

    let selected_label =
        fuzzy_select_with_key(labels_list, select_prompt_for("label")).and_then(|maybe_label| {
            maybe_label.ok_or_else(|| anyhow::anyhow!("Nothing selected. Aborting."))
        })?;

    let selected_update_fields = multi_fuzzy_select_with_key(
        EditableFields::iter().collect::<Vec<_>>(),
        select_prompt_for("options"),
        |_| false,
    )?;

    let edit_label_options = create_update_data(selected_update_fields, &selected_label)?;

    tracing::debug!("{edit_label_options:?}");

    let api_endpoint = EndpointGenerator::repo_labels_with_id(selected_label.id)?;

    let updated_label: Label = client.patch_body(api_endpoint, edit_label_options).await?;

    tracing::debug!("{updated_label:?}");

    Ok(())
}
fn create_update_data(
    selected_update_fields: Vec<EditableFields>,
    selected_label: &Label,
) -> anyhow::Result<EditLabelOption> {
    use EditableFields::*;

    let mut edit_label_options = EditLabelOption::from_label(selected_label);

    if selected_update_fields.contains(&Name) {
        let new_title = dialoguer::Input::new()
            .default(selected_label.name.to_owned())
            .with_prompt("Choose a new label name")
            .interact_text()?;
        edit_label_options.name.replace(new_title);
    }

    if selected_update_fields.contains(&Color) {
        let new_title = dialoguer::Input::new()
            .default(selected_label.color.to_owned())
            .with_prompt("Choose a new label color (format: #xxxxxx)")
            .interact_text()?;
        edit_label_options.color.replace(new_title);
    }

    if selected_update_fields.contains(&Description) {
        if let Some(new_description) =
            dialoguer::Editor::new().edit(selected_label.description.as_str())?
        {
            edit_label_options.description.replace(new_description);
        }
    }

    Ok(edit_label_options)
}
