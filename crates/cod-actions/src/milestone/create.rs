use cod_cli::milestone::create::CreateMilestoneArgs;
use cod_client::CodebergClient;
use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_types::api::create_options::create_milestone_option::CreateMilestoneOption;
use cod_types::api::milestone::Milestone;
use strum::Display;

pub async fn create_milestone(
    args: CreateMilestoneArgs,
    client: &CodebergClient,
) -> anyhow::Result<()> {
    let options = fill_in_mandatory_values(&args)?;
    let options = fill_in_optional_values(options, args)?;
    let api_endpoint = EndpointGenerator::repo_milestones()?;
    let response: Milestone = client.post_body(api_endpoint, options).await?;
    tracing::debug!("{response:?}");
    Ok(())
}

fn fill_in_mandatory_values(args: &CreateMilestoneArgs) -> anyhow::Result<CreateMilestoneOption> {
    let title = match args.title.clone() {
        Some(title) => title,
        None => dialoguer::Input::new()
            .with_prompt("Milestone Title")
            .interact()?,
    };
    Ok(CreateMilestoneOption::new(title))
}

fn fill_in_optional_values(
    mut options: CreateMilestoneOption,
    args: CreateMilestoneArgs,
) -> anyhow::Result<CreateMilestoneOption> {
    options = options.with_description(args.body.clone().unwrap_or_default());

    #[derive(Debug, Clone, Copy, Display, PartialEq, Eq)]
    enum PossiblyMissing {
        Description,
    }

    use PossiblyMissing::*;

    let missing_options = [args.body.is_none().then_some(Description)]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    if missing_options.is_empty() {
        return Ok(options);
    }

    if missing_options.contains(&Description) {
        let new_description = dialoguer::Editor::new()
            .edit("Enter a milestone description")?
            .ok_or_else(|| anyhow::anyhow!("Closed the editor. Aborting"))?;
        options = options.with_description(new_description);
    }

    Ok(options)
}
