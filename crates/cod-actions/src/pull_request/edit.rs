use cod_cli::pull_request::edit::EditPullRequestArgs;
use cod_client::CodebergClient;
use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_render::ui::{fuzzy_select_with_key, multi_fuzzy_select_with_key};
use cod_types::api::edit_pull_request_option::EditPullRequestOption;
use cod_types::api::pull_request::PullRequest;
use cod_types::api::state_type::StateType;
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Display, EnumIter, PartialEq, Eq)]
enum EditableFields {
    Assignees,
    Description,
    State,
    Title,
}

pub async fn edit_pull(_args: EditPullRequestArgs, client: &CodebergClient) -> anyhow::Result<()> {
    let list_pull_requests = client.get_repo_prs(None).await?;

    let selected_pull_request = fuzzy_select_with_key(
        list_pull_requests,
        |pr| format!("#{} {}", pr.number, pr.title),
        |pr| pr,
    )
    .and_then(|maybe_selected| {
        maybe_selected.ok_or_else(|| anyhow::anyhow!("Nothing selected. Aborting."))
    })?;

    let selected_update_fields = multi_fuzzy_select_with_key(
        EditableFields::iter().collect::<Vec<_>>(),
        |option| option.to_string(),
        |option| option,
        |_| false,
    )?;

    let edit_pull_request_options =
        create_update_data(client, selected_update_fields, &selected_pull_request).await?;

    let api_endpoint = EndpointGenerator::repo_update_issue(selected_pull_request.number)?;

    let updated_pull_request: PullRequest = client
        .patch_body(api_endpoint, edit_pull_request_options)
        .await?;

    tracing::info!("{updated_pull_request:?}");

    Ok(())
}

async fn create_update_data(
    client: &CodebergClient,
    selected_update_fields: Vec<EditableFields>,
    selected_pull_request: &PullRequest,
) -> anyhow::Result<EditPullRequestOption> {
    use EditableFields::*;

    let mut edit_pull_request_options =
        EditPullRequestOption::from_pull_request(selected_pull_request);

    if selected_update_fields.contains(&Assignees) {
        let assignees_list = client.get_repo_assignees().await?;
        let selected_assignees = multi_fuzzy_select_with_key(
            assignees_list,
            |assignee| assignee.username.to_owned(),
            |assignee| assignee.username,
            |assignee| {
                selected_pull_request
                    .assignees
                    .as_ref()
                    .map_or(false, |assignees| assignees.contains(assignee))
            },
        )?;
        edit_pull_request_options
            .assignees
            .replace(selected_assignees);
    }

    if selected_update_fields.contains(&Description) {
        let new_description = dialoguer::Editor::new().edit(selected_pull_request.body.as_str())?;
        edit_pull_request_options
            .body
            .replace(new_description.unwrap_or_default());
    }

    if selected_update_fields.contains(&State) {
        let new_state = fuzzy_select_with_key(
            StateType::available_for_choosing().to_vec(),
            |x| x.to_owned(),
            StateType::try_from,
        )?
        .and_then(|state_result| state_result.ok());
        edit_pull_request_options
            .state
            .replace(new_state.unwrap_or(selected_pull_request.state));
    }

    if selected_update_fields.contains(&Title) {
        let new_title = dialoguer::Input::new()
            .default(selected_pull_request.title.to_owned())
            .with_prompt("Choose a new issue title")
            .interact_text()?;
        edit_pull_request_options.title.replace(new_title);
    }

    Ok(edit_pull_request_options)
}
