use crate::cli::issue::edit::EditIssueArgs;
use crate::client::BergClient;
use crate::endpoints::endpoint_generator::EndpointGenerator;
use crate::render::spinner::spin_until_ready;
use crate::render::ui::{fuzzy_select_with_key, multi_fuzzy_select_with_key};
use crate::types::api::edit_options::edit_issue_option::EditIssueOption;
use crate::types::api::issue::Issue;
use crate::types::api::issue_labels_option::IssueLabelsOption;
use crate::types::api::label::Label;
use crate::types::api::state_type::StateType;
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::actions::text_manipulation::{edit_prompt_for, input_prompt_for, select_prompt_for};

#[derive(Display, EnumIter, PartialEq, Eq)]
enum EditableFields {
    Assignees,
    Description,
    State,
    Title,
    Labels,
}

pub async fn edit_issue(_args: EditIssueArgs, client: &BergClient) -> anyhow::Result<()> {
    let issues_list = spin_until_ready(client.get_repo_issues(None, None)).await?;

    if issues_list.is_empty() {
        println!("No issues found in this repository");
    }

    let selected_issue =
        fuzzy_select_with_key(issues_list, select_prompt_for("issue")).and_then(|maybe_issue| {
            maybe_issue.ok_or_else(|| anyhow::anyhow!("Nothing selected. Aborting."))
        })?;

    let selected_update_fields = multi_fuzzy_select_with_key(
        EditableFields::iter().collect::<Vec<_>>(),
        select_prompt_for("options"),
        |_| false,
    )?;

    let (edit_issue_options, issue_labels_option) =
        create_update_data(client, selected_update_fields, &selected_issue).await?;

    if let Some(options) = issue_labels_option {
        tracing::debug!("{options:?}");
        let replaced_labels: Vec<Label> = client
            .replace_labels(selected_issue.number, options)
            .await?;
        tracing::debug!("{replaced_labels:?}");
    }

    tracing::debug!("{edit_issue_options:?}");

    let api_endpoint = EndpointGenerator::repo_update_issue(selected_issue.number)?;
    let updated_issue: Issue = client.patch_body(api_endpoint, edit_issue_options).await?;

    tracing::debug!("{updated_issue:?}");

    Ok(())
}

async fn create_update_data(
    client: &BergClient,
    selected_update_fields: Vec<EditableFields>,
    selected_issue: &Issue,
) -> anyhow::Result<(EditIssueOption, Option<IssueLabelsOption>)> {
    use EditableFields::*;

    let mut edit_issue_options = EditIssueOption::from_issue(selected_issue);

    if selected_update_fields.contains(&Assignees) {
        let assignees_list = client.get_repo_assignees().await?;
        let selected_assignees = multi_fuzzy_select_with_key(
            assignees_list,
            select_prompt_for("assignees"),
            |assignee| {
                selected_issue
                    .assignees
                    .as_ref()
                    .map_or(false, |assignees| assignees.contains(assignee))
            },
        )?;
        edit_issue_options.assignees.replace(
            selected_assignees
                .into_iter()
                .map(|assignee| assignee.username)
                .collect::<Vec<_>>(),
        );
    }

    let issue_labels_option = if selected_update_fields.contains(&Labels) {
        let labels_list = client.get_repo_labels(None).await?;
        let selected_labels =
            multi_fuzzy_select_with_key(labels_list, select_prompt_for("labels"), |label| {
                selected_issue.labels.contains(label)
            })?;
        Some(IssueLabelsOption {
            labels: selected_labels
                .into_iter()
                .map(|label| label.id)
                .collect::<Vec<_>>(),
        })
    } else {
        None
    };

    if selected_update_fields.contains(&Description) {
        let new_description =
            inquire::Editor::new(edit_prompt_for("the new issue description").as_str())
                .with_predefined_text(selected_issue.body.as_str())
                .prompt()?;
        edit_issue_options.body.replace(new_description);
    }

    if selected_update_fields.contains(&State) {
        let new_state = fuzzy_select_with_key(
            StateType::available_for_choosing().to_vec(),
            select_prompt_for("state"),
        )?;
        edit_issue_options
            .state
            .replace(new_state.unwrap_or(selected_issue.state));
    }

    if selected_update_fields.contains(&Title) {
        let new_title = inquire::Text::new(input_prompt_for("Choose a new issue title").as_str())
            .with_default(selected_issue.title.as_str())
            .prompt()?;
        edit_issue_options.title.replace(new_title);
    }

    Ok((edit_issue_options, issue_labels_option))
}
