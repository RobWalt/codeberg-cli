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

    let base = args
        .base
        .as_ref()
        .map(|base| Ok(base.to_owned()))
        .unwrap_or_else(get_current_base)?;

    let head = select_head(base.as_str(), args, client).await?;

    Ok(CreatePullRequestOption::new(title, base, head))
}

async fn select_head(
    base: &str,
    args: &CreatePullRequestArgs,
    client: &CodebergClient,
) -> anyhow::Result<String> {
    if let Some(head) = args.head.clone() {
        Ok(head)
    } else {
        let branches = client
            .get_repo_branches()
            .await?
            .into_iter()
            .filter(|branch| branch.name != base)
            .collect::<Vec<_>>();

        fuzzy_select_with_key(
            branches,
            select_prompt_for("source branch"),
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

fn get_current_base() -> anyhow::Result<String> {
    let output = std::process::Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .output()?;
    String::from_utf8(output.stdout)
        .map(|base| base.trim().to_owned())
        .map_err(anyhow::Error::from)
}
