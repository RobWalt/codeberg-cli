use cod_cli::pull_request::create::CreatePullRequestArgs;
use cod_client::CodebergClient;
use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_render::ui::{
    fuzzy_select_with_key, fuzzy_select_with_key_with_default, multi_fuzzy_select_with_key,
};
use cod_types::api::create_options::create_pull_request_option::CreatePullRequestOption;
use cod_types::api::pull_request::PullRequest;
use cod_types::api::state_type::StateType;
use strum::Display;

use crate::text_manipulation::select_prompt_for;

pub async fn create_pull(
    args: CreatePullRequestArgs,
    client: &CodebergClient,
) -> anyhow::Result<()> {
    let options = fill_in_mandatory_values(&args, client).await?;
    let options = fill_in_optional_values(options, &args, client).await?;
    let api_endpoint = EndpointGenerator::repo_pull_requests()?;
    let response: PullRequest = client.post_body(api_endpoint, options).await?;
    tracing::debug!("{response:?}");
    Ok(())
}

async fn fill_in_mandatory_values(
    args: &CreatePullRequestArgs,
    client: &CodebergClient,
) -> anyhow::Result<CreatePullRequestOption> {
    let title = dialoguer::Input::new()
        .with_prompt("Pull Request Title")
        .interact_text()?;

    let target_branch = select_branch(
        None,
        "target branch into which changes are merged",
        vec!["main", "master"],
        args,
        client,
    )
    .await?;

    let current_checkout = get_current_checkout()?;
    let source_branch = select_branch(
        Some(target_branch.as_str()),
        "source branch containing the changes",
        vec![current_checkout.as_str()],
        args,
        client,
    )
    .await?;

    Ok(CreatePullRequestOption::new(
        title,
        source_branch,
        target_branch,
    ))
}

async fn select_branch(
    filter_branch: Option<&str>,
    prompt_text: &str,
    default_branch_names: Vec<&str>,
    args: &CreatePullRequestArgs,
    client: &CodebergClient,
) -> anyhow::Result<String> {
    if let Some(target_branch) = args.target_branch.clone() {
        Ok(target_branch)
    } else {
        let branches = client
            .get_repo_branches()
            .await?
            .into_iter()
            .filter(|branch| {
                filter_branch.map_or(true, |filter_name| branch.name.as_str() != filter_name)
            })
            .collect::<Vec<_>>();

        if branches.is_empty() {
            anyhow::bail!("No branches except {filter_branch:?} found. Maybe the branch you want to merge doesn't exist on remote yet?");
        }

        tracing::debug!("branches:{branches:?}");

        let default_index = default_branch_names.iter().find_map(|&default_name| {
            branches
                .iter()
                .position(|branch| branch.name.as_str() == default_name)
        });

        tracing::debug!("default_idx:{default_index:?}");

        let default_index = default_branch_names.iter().find_map(|&default_name| {
            branches
                .iter()
                .position(|branch| branch.name.as_str() == default_name)
        });

        fuzzy_select_with_key_with_default(
            branches,
            select_prompt_for(prompt_text),
            |branch| branch.name.to_owned(),
            |branch| branch.name,
            default_index,
        )
        .and_then(|maybe_selection| {
            maybe_selection.ok_or_else(|| anyhow::anyhow!("No valid target selected. Maybe the branch doesn't exist on remote yet. Aborting."))
        })
        .map_err(anyhow::Error::from)
    }
}

async fn fill_in_optional_values(
    mut options: CreatePullRequestOption,
    args: &CreatePullRequestArgs,
    client: &CodebergClient,
) -> anyhow::Result<CreatePullRequestOption> {
    options = options
        .with_description(args.body.clone().unwrap_or_default())
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

    let selected_options = multi_fuzzy_select_with_key(
        missing_options,
        select_prompt_for("options"),
        |&missing_option| missing_option,
        |missing_option| missing_option,
        |_| false,
    )?;

    if selected_options.contains(&Description) {
        options = options.with_description(
            dialoguer::Editor::new()
                .edit("Enter a pull request description")?
                .unwrap_or_default(),
        );
    }

    if selected_options.contains(&Assignees) {
        let assignees_list = client.get_repo_assignees().await?;
        let selected_assignees = multi_fuzzy_select_with_key(
            assignees_list,
            select_prompt_for("assignees"),
            |assignee| assignee.username.to_owned(),
            |assignee| assignee.username,
            |_| false,
        )?;

        options = options.with_assignees(selected_assignees);
    }

    if selected_options.contains(&Labels) {
        let labels_list = client.get_repo_labels(None).await?;

        let selected_labels = multi_fuzzy_select_with_key(
            labels_list,
            select_prompt_for("labels"),
            |label| label.name.to_owned(),
            |label| label.id,
            |_| false,
        )?;

        options = options.with_labels(selected_labels);
    }

    if selected_options.contains(&Milestone) {
        let milstones_list = client
            .get_repo_milestones(Some(StateType::Open), None)
            .await?;

        let selected_milestone = fuzzy_select_with_key(
            milstones_list,
            select_prompt_for("milestone"),
            |milestone| milestone.title.to_owned(),
            |milestone| milestone.id,
        )?
        .ok_or_else(|| anyhow::anyhow!("No milestone selected. Aborting."))?;

        options = options.with_milestone(selected_milestone);
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

fn get_current_checkout() -> anyhow::Result<String> {
    let output = std::process::Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .output()?;
    String::from_utf8(output.stdout)
        .map(|base| base.trim().to_owned())
        .map_err(anyhow::Error::from)
}
