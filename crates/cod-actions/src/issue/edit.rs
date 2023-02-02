use cod_cli::issue::edit::EditIssueArgs;
use cod_client::CodebergClient;
use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_render::spinner::spin_until_ready;
use cod_render::ui::{fuzzy_select_with_key, multi_fuzzy_select_with_key};
use cod_types::api::edit_issue_option::EditIssueOption;
use cod_types::api::issue::Issue;
use cod_types::api::state_type::StateType;
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::text_manipulation::select_prompt_for;

#[derive(Display, EnumIter, PartialEq, Eq)]
enum EditableFields {
    Assignees,
    Description,
    State,
    Title,
}

pub async fn edit_issue(_args: EditIssueArgs, client: &CodebergClient) -> anyhow::Result<()> {
    let issues_list = spin_until_ready(client.get_repo_issues(None, None)).await?;

    if issues_list.is_empty() {
        println!("No issues found in this repository");
    }

    let selected_issue = fuzzy_select_with_key(
        issues_list,
        select_prompt_for("issue"),
        |issue: &Issue| format!("#{} {}", issue.number, issue.title),
        |issue| issue,
    )
    .and_then(|maybe_issue| {
        maybe_issue.ok_or_else(|| anyhow::anyhow!("Nothing selected. Aborting."))
    })?;

    let selected_update_fields = multi_fuzzy_select_with_key(
        EditableFields::iter().collect::<Vec<_>>(),
        select_prompt_for("options"),
        |option| option.to_string(),
        |option| option,
        |_| false,
    )?;

    let edit_issue_options =
        create_update_data(client, selected_update_fields, &selected_issue).await?;

    tracing::debug!("{edit_issue_options:?}");

    let api_endpoint = EndpointGenerator::repo_update_issue(selected_issue.number)?;

    let updated_issue: Issue = client.patch_body(api_endpoint, edit_issue_options).await?;

    tracing::debug!("{updated_issue:?}");

    Ok(())
}

async fn create_update_data(
    client: &CodebergClient,
    selected_update_fields: Vec<EditableFields>,
    selected_issue: &Issue,
) -> anyhow::Result<EditIssueOption> {
    use EditableFields::*;

    let mut edit_issue_options = EditIssueOption::from_issue(selected_issue);

    if selected_update_fields.contains(&Assignees) {
        let assignees_list = client.get_repo_assignees().await?;
        let selected_assignees = multi_fuzzy_select_with_key(
            assignees_list,
            select_prompt_for("assignees"),
            |assignee| assignee.username.to_owned(),
            |assignee| assignee.username,
            |assignee| {
                selected_issue
                    .assignees
                    .as_ref()
                    .map_or(false, |assignees| assignees.contains(assignee))
            },
        )?;
        edit_issue_options.assignees.replace(selected_assignees);
    }

    if selected_update_fields.contains(&Description) {
        if let Some(new_description) =
            dialoguer::Editor::new().edit(selected_issue.body.as_str())?
        {
            edit_issue_options.body.replace(new_description);
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
        edit_issue_options
            .state
            .replace(new_state.unwrap_or(selected_issue.state));
    }

    if selected_update_fields.contains(&Title) {
        let new_title = dialoguer::Input::new()
            .default(selected_issue.title.to_owned())
            .with_prompt("Choose a new issue title")
            .interact_text()?;
        edit_issue_options.title.replace(new_title);
    }

    Ok(edit_issue_options)
}
