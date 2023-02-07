use cod_cli::milestone::edit::EditMilestoneArgs;
use cod_client::CodebergClient;
use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_render::spinner::spin_until_ready;
use cod_render::ui::fuzzy_select_with_key;
use cod_render::ui::multi_fuzzy_select_with_key;
use cod_types::api::edit_options::edit_milestone_option::EditMilestoneOption;
use cod_types::api::milestone::Milestone;
use cod_types::api::state_type::StateType;
use strum::IntoEnumIterator;
use strum::{Display, EnumIter};

use crate::text_manipulation::select_prompt_for;

#[derive(Display, EnumIter, PartialEq, Eq)]
enum EditableFields {
    Description,
    State,
    Title,
}

pub async fn edit_milestone(
    _args: EditMilestoneArgs,
    client: &CodebergClient,
) -> anyhow::Result<()> {
    let milestones_list = spin_until_ready(client.get_repo_milestones(None, None)).await?;

    if milestones_list.is_empty() {
        println!("No milestones found in this repository");
    }

    let selected_milestone = fuzzy_select_with_key(
        milestones_list,
        select_prompt_for("milestone"),
        |milestone: &Milestone| milestone.title.clone(),
        |milestone| milestone,
    )?
    .ok_or_else(|| anyhow::anyhow!("Nothing selected. Aborting."))?;

    let selected_update_fields = multi_fuzzy_select_with_key(
        EditableFields::iter().collect::<Vec<_>>(),
        select_prompt_for("options"),
        |option| option.to_string(),
        |option| option,
        |_| false,
    )?;

    let edit_milestone_options = create_update_data(selected_update_fields, &selected_milestone)?;

    tracing::debug!("{edit_milestone_options:?}");

    let api_endpoint = EndpointGenerator::repo_update_milestone(selected_milestone.id)?;

    let updated_milestone: Milestone = client
        .patch_body(api_endpoint, edit_milestone_options)
        .await?;

    tracing::debug!("{updated_milestone:?}");

    Ok(())
}

fn create_update_data(
    selected_update_fields: Vec<EditableFields>,
    selected_milestone: &Milestone,
) -> anyhow::Result<EditMilestoneOption> {
    use EditableFields::*;

    let mut edit_milestone_options = EditMilestoneOption::from_milestone(selected_milestone);

    if selected_update_fields.contains(&Description) {
        if let Some(new_description) = dialoguer::Editor::new().edit(
            selected_milestone
                .description
                .as_deref()
                .unwrap_or_default(),
        )? {
            edit_milestone_options.description.replace(new_description);
        }
    }

    if selected_update_fields.contains(&State) {
        let new_state = fuzzy_select_with_key(
            StateType::available_for_choosing().to_vec(),
            select_prompt_for("state"),
            |state| state.to_owned(),
            StateType::try_from,
        )?
        .and_then(|state_result| state_result.ok());
        edit_milestone_options
            .state
            .replace(new_state.unwrap_or(selected_milestone.state));
    }

    if selected_update_fields.contains(&Title) {
        let new_title = dialoguer::Input::new()
            .default(selected_milestone.title.to_owned())
            .with_prompt("Choose a new milestone title")
            .interact_text()?;
        edit_milestone_options.title.replace(new_title);
    }

    Ok(edit_milestone_options)
}
