use cod_cli::issue::create::CreateIssueArgs;
use cod_client::CodebergClient;
use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_render::ui::multi_fuzzy_select_with_key;
use cod_types::api::create_issue_options::CreateIssueOption;
use cod_types::api::issue::Issue;
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
        "Add additional information for",
        |&missing_option| missing_option,
        |missing_option| missing_option,
        |_| false,
    )?;

    if selected_options.contains(&Description) {
        let new_body = dialoguer::Editor::new()
            .edit("Enter a issue description")?
            .ok_or_else(|| anyhow::anyhow!("Closed the editor. Aborting."))?;
        options = options.with_body(new_body);
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
            |label| label.name,
            |_| false,
        )?;

        options = options.with_labels(id_for_labels(client, Some(selected_labels.as_ref())).await?);
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
