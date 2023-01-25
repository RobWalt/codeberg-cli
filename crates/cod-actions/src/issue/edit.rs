use cod_cli::issue::edit::EditIssueArgs;
use cod_client::CodebergClient;
use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_render::spinner::spin_until_ready;
use cod_render::ui::{fuzzy_select_with_key, multi_fuzzy_select_with_key};
use cod_types::api::edit_issue_option::EditIssueOptions;
use cod_types::api::issue::Issue;
use cod_types::api::state_type::StateType;
use cod_types::api::user::User;
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Display, EnumIter, PartialEq, Eq)]
enum EditableFields {
    Assignees,
    Description,
    State,
    Title,
}

pub async fn edit_issue(_args: EditIssueArgs, client: &CodebergClient) -> anyhow::Result<()> {
    let issues_list = spin_until_ready(get_issue_list(client)).await?;

    if issues_list.is_empty() {
        println!("No issues found in this repository");
    }

    let selected_issue = fuzzy_select_with_key(
        issues_list,
        |issue: &Issue| format!("#{} {}", issue.number, issue.title),
        |issue| issue,
    )
    .and_then(|maybe_issue| maybe_issue.ok_or_else(|| anyhow::anyhow!("No issues")))?;

    let selected_update_fields = multi_fuzzy_select_with_key(
        EditableFields::iter().collect::<Vec<_>>(),
        |option| option.to_string(),
        |option| option,
        |_| false,
    )?;

    let edit_issue_options =
        create_update_data(client, selected_update_fields, &selected_issue).await?;

    tracing::info!("{edit_issue_options:?}");

    let api_endpoint = EndpointGenerator::repo_update_issue(selected_issue.number)?;

    let updated_issue: Issue = client.patch_body(api_endpoint, edit_issue_options).await?;

    tracing::info!("{updated_issue:?}");

    Ok(())
}

async fn get_issue_list(client: &CodebergClient) -> anyhow::Result<Vec<Issue>> {
    let api_endpoint = EndpointGenerator::repo_issues()?;
    client.get::<Vec<Issue>>(api_endpoint).await
}

async fn create_update_data(
    client: &CodebergClient,
    selected_update_fields: Vec<EditableFields>,
    selected_issue: &Issue,
) -> anyhow::Result<EditIssueOptions> {
    use EditableFields::*;

    let mut edit_issue_options = EditIssueOptions::from_issue(selected_issue);

    if selected_update_fields.contains(&Assignees) {
        let assignees_list = get_assignees(client).await?;
        let selected_assignees = multi_fuzzy_select_with_key(
            assignees_list,
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
        let new_description = dialoguer::Editor::new().edit(selected_issue.body.as_str())?;
        edit_issue_options
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

async fn get_assignees(client: &CodebergClient) -> anyhow::Result<Vec<User>> {
    let api_endpoint = EndpointGenerator::repo_assignees()?;
    let repo_assignees = client.get(api_endpoint).await?;
    Ok(repo_assignees)
}