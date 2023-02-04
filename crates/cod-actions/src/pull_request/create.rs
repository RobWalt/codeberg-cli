use cod_cli::pull_request::create::CreatePullRequestArgs;
use cod_client::CodebergClient;
use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_render::ui::{fuzzy_select_with_key, multi_fuzzy_select_with_key};
use cod_types::api::create_pull_request_option::CreatePullRequestOption;
use cod_types::api::pull_request::PullRequest;
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
        args,
        client,
    )
    .await?;

    let source_branch = select_branch(
        Some(target_branch.as_str()),
        "source branch containing the changes",
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
    filter_branch_out: Option<&str>,
    prompt_text: &str,
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
                filter_branch_out.map_or(true, |filter_name| branch.name.as_str() != filter_name)
            })
            .collect::<Vec<_>>();

        fuzzy_select_with_key(
            branches,
            select_prompt_for(prompt_text),
            |branch| branch.name.to_owned(),
            |branch| branch.name,
        )
        .and_then(|maybe_selection| {
            maybe_selection.ok_or_else(|| anyhow::anyhow!("No valid target selected. Aborting."))
        })
        .map_err(anyhow::Error::from)
    }
}

async fn fill_in_optional_values(
    mut options: CreatePullRequestOption,
    args: &CreatePullRequestArgs,
    client: &CodebergClient,
) -> anyhow::Result<CreatePullRequestOption> {
    #[derive(Debug, Clone, Copy, Display, PartialEq, Eq)]
    enum PossiblyMissing {
        Description,
        Assignees,
        Labels,
    }
    use PossiblyMissing::*;
    let missing_options = [
        args.body.is_none().then_some(Description),
        args.assignees.is_none().then_some(Assignees),
        args.labels.is_none().then_some(Labels),
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

    Ok(options)
}
