use cod_cli::issue::create::CreateIssueArgs;
use cod_client::CodebergClient;
use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_render::ui::multi_fuzzy_select_with_key;
use cod_types::api::create_issue_options::CreateIssueOption;
use cod_types::api::issue::Issue;
use strum::Display;

pub async fn create_issue(
    mut args: CreateIssueArgs,
    client: &CodebergClient,
) -> anyhow::Result<()> {
    args = fill_in_mandatory_values(args)?;
    args = fill_in_optional_values(args, client).await?;
    let body = create_body(args);
    let api_endpoint = EndpointGenerator::repo_issues()?;
    let response: Issue = client.post_body(api_endpoint, body).await?;
    tracing::debug!("{response:?}");
    Ok(())
}

fn fill_in_mandatory_values(mut args: CreateIssueArgs) -> anyhow::Result<CreateIssueArgs> {
    if args.title.is_none() {
        args.title.replace(
            dialoguer::Input::new()
                .with_prompt("Issue Title")
                .interact()?,
        );
    }
    Ok(args)
}

async fn fill_in_optional_values(
    mut args: CreateIssueArgs,
    client: &CodebergClient,
) -> anyhow::Result<CreateIssueArgs> {
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
        return Ok(args);
    }

    let selected_options = multi_fuzzy_select_with_key(
        missing_options,
        |&missing_option| missing_option,
        |missing_option| missing_option,
        |_| false,
    )?;

    if selected_options.contains(&Description) {
        args.body = dialoguer::Editor::new().edit("Enter a issue description")?;
    }

    if selected_options.contains(&Assignees) {
        let assignees_list = client.get_repo_assignees().await?;
        let selected_assignees = multi_fuzzy_select_with_key(
            assignees_list,
            |assignee| assignee.username.to_owned(),
            |assignee| assignee.username,
            |_| false,
        )?;

        args.assignees.replace(selected_assignees);
    }

    if selected_options.contains(&Labels) {
        let labels_list = client.get_repo_labels(None).await?;

        let selected_labels = multi_fuzzy_select_with_key(
            labels_list,
            |label| label.name.to_owned(),
            |label| label.id,
            |_| false,
        )?;

        args.labels.replace(selected_labels);
    }

    Ok(args)
}

fn create_body(args: CreateIssueArgs) -> CreateIssueOption {
    CreateIssueOption::new(args.title.unwrap_or_default())
        .with_body(args.body.unwrap_or_default())
        .with_assignees(args.assignees.unwrap_or_default())
        .with_labels(args.labels.unwrap_or_default())
}
