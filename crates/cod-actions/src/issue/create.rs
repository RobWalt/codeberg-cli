use cod_cli::issue::create::CreateIssueArgs;
use cod_client::CodebergClient;
use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_render::ui::{fuzzy_select_with_key, multi_fuzzy_select_with_key};
use cod_types::api::create_options::create_issue_options::CreateIssueOption;
use cod_types::api::issue::Issue;
use cod_types::api::state_type::StateType;
use strum::Display;

use crate::text_manipulation::select_prompt_for;

pub async fn create_issue(args: CreateIssueArgs, client: &CodebergClient) -> anyhow::Result<()> {
    let options = fill_in_mandatory_values(&args)?;
    let options = fill_in_optional_values(options, args, client).await?;
    let api_endpoint = EndpointGenerator::repo_issues()?;
    let response: Issue = client.post_body(api_endpoint, options).await?;
    tracing::debug!("{response:?}");
    Ok(())
}

fn fill_in_mandatory_values(args: &CreateIssueArgs) -> anyhow::Result<CreateIssueOption> {
    let title = match args.title.clone() {
        Some(title) => title,
        None => dialoguer::Input::new()
            .with_prompt("Issue Title")
            .interact()?,
    };
    Ok(CreateIssueOption::new(title))
}

async fn fill_in_optional_values(
    mut options: CreateIssueOption,
    args: CreateIssueArgs,
    client: &CodebergClient,
) -> anyhow::Result<CreateIssueOption> {
    options = options
        .with_body(args.body.clone().unwrap_or_default())
        .with_labels(id_for_labels(client, args.labels.as_ref()).await?)
        .with_assignees(args.assignees.clone().unwrap_or_default());

    // set milestone if given as a CLI option
    if let Some(ref milestone_name) = args.milestone {
        let milestone_id = id_for_milestone(client, milestone_name)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Couldn't find milestone with name {milestone_name}"))?;
        options = options.with_milestone(milestone_id);
    }

    #[derive(Debug, Clone, Copy, Display, PartialEq, Eq)]
    enum PossiblyMissing {
        Description,
        Assignees,
        Labels,
        Milestone,
    }

    use PossiblyMissing::*;

    let missing_options = [
        args.body.is_none().then_some(Description),
        args.assignees.is_none().then_some(Assignees),
        args.labels.is_none().then_some(Labels),
        args.milestone.is_none().then_some(Milestone),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    if missing_options.is_empty() {
        return Ok(options);
    }

    let selected_options =
        multi_fuzzy_select_with_key(missing_options, "Add additional information for", |_| false)?;

    if selected_options.contains(&Description) {
        let new_body = dialoguer::Editor::new()
            .edit("Enter a issue description")?
            .ok_or_else(|| anyhow::anyhow!("Closed the editor. Aborting."))?;
        options = options.with_body(new_body);
    }

    if selected_options.contains(&Assignees) {
        let assignees_list = client.get_repo_assignees().await?;
        let selected_assignees =
            multi_fuzzy_select_with_key(assignees_list, select_prompt_for("assignees"), |_| false)?;

        options = options.with_assignees(
            selected_assignees
                .into_iter()
                .map(|user| user.username)
                .collect::<Vec<_>>(),
        );
    }

    if selected_options.contains(&Labels) {
        let labels_list = client.get_repo_labels(None).await?;

        let selected_labels =
            multi_fuzzy_select_with_key(labels_list, select_prompt_for("labels"), |_| false)?;

        options = options.with_labels(
            selected_labels
                .into_iter()
                .map(|label| label.id)
                .collect::<Vec<_>>(),
        );
    }

    if selected_options.contains(&Milestone) {
        let milstones_list = client
            .get_repo_milestones(Some(StateType::Open), None)
            .await?;

        let selected_milestone =
            fuzzy_select_with_key(milstones_list, select_prompt_for("milestone"))?
                .ok_or_else(|| anyhow::anyhow!("No milestone selected. Aborting."))?;

        options = options.with_milestone(selected_milestone.id);
    }

    Ok(options)
}

async fn id_for_labels(
    client: &CodebergClient,
    labels: Option<&Vec<String>>,
) -> anyhow::Result<Vec<usize>> {
    let labels = match labels {
        Some(labels) => {
            let labels_list = client.get_repo_labels(None).await?;
            labels
                .iter()
                .filter_map(|label_name| {
                    labels_list
                        .iter()
                        .find(|label| label.name.as_str() == label_name.as_str())
                        .map(|label| label.id)
                })
                .collect::<Vec<_>>()
        }
        None => vec![],
    };
    Ok(labels)
}

async fn id_for_milestone(
    client: &CodebergClient,
    milestone_name: &str,
) -> anyhow::Result<Option<usize>> {
    let milestone_list = client
        .get_repo_milestones(Some(StateType::Open), None)
        .await?;
    let maybe_milestone_id = milestone_list
        .into_iter()
        .find(|milestone| milestone.title == milestone_name)
        .map(|milestone| milestone.id);
    Ok(maybe_milestone_id)
}
