use crate::cli::milestone::edit::EditMilestoneArgs;
use crate::client::BergClient;
use crate::endpoints::endpoint_generator::EndpointGenerator;
use crate::render::spinner::spin_until_ready;
use crate::render::ui::fuzzy_select_with_key;
use crate::render::ui::multi_fuzzy_select_with_key;
use crate::types::api::edit_options::edit_milestone_option::EditMilestoneOption;
use crate::types::api::milestone::Milestone;
use crate::types::api::state_type::StateType;
use strum::IntoEnumIterator;
use strum::{Display, EnumIter};

use crate::actions::text_manipulation::edit_prompt_for;
use crate::actions::text_manipulation::input_prompt_for;
use crate::actions::text_manipulation::select_prompt_for;

#[derive(Display, EnumIter, PartialEq, Eq)]
enum EditableFields {
    Description,
    State,
    Title,
}

pub async fn edit_milestone(_args: EditMilestoneArgs, client: &BergClient) -> anyhow::Result<()> {
    let milestones_list = spin_until_ready(client.get_repo_milestones(None, None)).await?;

    if milestones_list.is_empty() {
        println!("No milestones found in this repository");
    }

    let selected_milestone =
        fuzzy_select_with_key(milestones_list, select_prompt_for("milestone"))?
            .ok_or_else(|| anyhow::anyhow!("Nothing selected. Aborting."))?;

    let selected_update_fields = multi_fuzzy_select_with_key(
        EditableFields::iter().collect::<Vec<_>>(),
        select_prompt_for("options"),
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
        let new_description =
            inquire::Editor::new(edit_prompt_for("a milestone description").as_str())
                .with_predefined_text(
                    selected_milestone
                        .description
                        .as_deref()
                        .unwrap_or_default(),
                )
                .prompt()?;
        edit_milestone_options.description.replace(new_description);
    }

    if selected_update_fields.contains(&State) {
        let new_state = fuzzy_select_with_key(
            StateType::available_for_choosing().to_vec(),
            select_prompt_for("state"),
        )?;
        edit_milestone_options
            .state
            .replace(new_state.unwrap_or(selected_milestone.state));
    }

    if selected_update_fields.contains(&Title) {
        let new_title =
            inquire::Text::new(input_prompt_for("Choose a new milestone title").as_str())
                .with_default(selected_milestone.title.as_str())
                .prompt()?;
        edit_milestone_options.title.replace(new_title);
    }

    Ok(edit_milestone_options)
}
