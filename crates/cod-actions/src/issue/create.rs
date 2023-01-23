use cod_cli::issue::create::CreateIssueArgs;
use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_types::api::label::Label;
use cod_types::api::user::User;
use cod_types::client::CodebergClient;
use cod_types::token::Token;
use strum::Display;

pub async fn create_issue(mut args: CreateIssueArgs, token: Token) -> anyhow::Result<()> {
    let client = CodebergClient::new(&token)?;
    args = fill_in_mandatory_values(args)?;
    args = fill_in_optional_values(args, &client).await?;
    tracing::info!("{args:?}");
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
    #[derive(Debug, Display, PartialEq, Eq)]
    enum PossiblyMissing {
        Description,
        Assignees,
        Labels,
    }
    use PossiblyMissing::*;
    let missing_fields = [
        args.body.is_none().then_some(Description),
        args.assignees.is_none().then_some(Assignees),
        args.labels.is_none().then_some(Labels),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    if missing_fields.is_empty() {
        return Ok(args);
    }

    let selected_options = dialoguer::MultiFuzzySelect::new()
        .items(&missing_fields)
        .interact()
        .map(|item_idxs| {
            missing_fields
                .into_iter()
                .enumerate()
                .filter(|(idx, _)| item_idxs.contains(idx))
                .map(|(_, option)| option)
                .collect::<Vec<_>>()
        })?;

    if selected_options.contains(&Description) {
        args.body = dialoguer::Editor::new().edit("Enter a issue description")?;
    }

    if selected_options.contains(&Assignees) {
        let api_endpoint_assignees = EndpointGenerator::repo_assignees()?;
        let assignees_list =
            client
                .get::<Vec<User>>(api_endpoint_assignees)
                .await
                .map(|assignees| {
                    assignees
                        .into_iter()
                        .map(|assignee| assignee.username)
                        .collect::<Vec<_>>()
                })?;
        let selected_assignees = dialoguer::MultiFuzzySelect::new()
            .items(&assignees_list)
            .interact()
            .map(|idxs| {
                assignees_list
                    .into_iter()
                    .enumerate()
                    .filter(|(idx, _)| idxs.contains(idx))
                    .map(|(_, assignee)| assignee)
                    .collect::<Vec<_>>()
            })?;

        args.assignees.replace(selected_assignees);
    }

    if selected_options.contains(&Labels) {
        let api_endpoint_assignees = EndpointGenerator::repo_labels()?;

        let labels_list = client.get::<Vec<Label>>(api_endpoint_assignees).await?;

        let selected_labels = dialoguer::MultiFuzzySelect::new()
            .items(
                &labels_list
                    .iter()
                    .map(|label| label.name.as_str())
                    .collect::<Vec<_>>(),
            )
            .interact()
            .map(|idxs| {
                labels_list
                    .into_iter()
                    .enumerate()
                    .filter(|(idx, _)| idxs.contains(idx))
                    .map(|(_, label)| label.id)
                    .collect::<Vec<_>>()
            })?;

        args.labels.replace(selected_labels);
    }

    Ok(args)
}
