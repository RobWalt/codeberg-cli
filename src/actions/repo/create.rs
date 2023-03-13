use crate::cli::repo::create::RepoCreateArgs;
use crate::client::BergClient;
use crate::endpoints::endpoint_generator::EndpointGenerator;
use crate::render::ui::{fuzzy_select_with_key, multi_fuzzy_select_with_key};
use crate::types::api::create_options::create_repo_options::CreateRepoOption;
use crate::types::api::privacy_type::Privacy;
use crate::types::api::repository::Repository;
use inquire::validator::Validation;
use strum::{Display, IntoEnumIterator};

use crate::actions::text_manipulation::{edit_prompt_for, input_prompt_for, select_prompt_for};

pub async fn create_repo(mut args: RepoCreateArgs, client: &BergClient) -> anyhow::Result<()> {
    args = fill_in_mandatory_values(args)?;
    args = fill_in_optional_values(args)?;
    let body = create_body(args);
    let api_endpoint = EndpointGenerator::user_repos()?;
    let response: Repository = client.post_body(api_endpoint, body).await?;
    tracing::debug!("{response:?}");
    Ok(())
}

fn fill_in_mandatory_values(mut args: RepoCreateArgs) -> anyhow::Result<RepoCreateArgs> {
    if args.name.is_none() {
        args.name.replace(
            inquire::Text::new(input_prompt_for("Repository Name").as_str())
                .with_validator(|input: &str| {
                    if input.chars().any(|char| char.is_whitespace()) {
                        Ok(Validation::Invalid(
                            "Whitespace not allowed in repository name.".into(),
                        ))
                    } else {
                        Ok(Validation::Valid)
                    }
                })
                .prompt()?,
        );
    }
    Ok(args)
}

fn fill_in_optional_values(mut args: RepoCreateArgs) -> anyhow::Result<RepoCreateArgs> {
    #[derive(Debug, Clone, Copy, Display, PartialEq, Eq)]
    enum PossiblyMissing {
        DefaultBranch,
        Description,
        Private,
    }

    use PossiblyMissing::*;
    let missing_options = [
        args.default_branch.is_none().then_some(DefaultBranch),
        args.description.is_none().then_some(Description),
        args.private.is_none().then_some(Private),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    if missing_options.is_empty() {
        return Ok(args);
    }

    let selected_options =
        multi_fuzzy_select_with_key(missing_options, select_prompt_for("option"), |_| false)?;

    if selected_options.contains(&DefaultBranch) {
        args.default_branch.replace(
            inquire::Text::new(input_prompt_for("Repository default branch").as_str())
                .with_default("main")
                .prompt()?,
        );
    }

    if selected_options.contains(&Description) {
        let new_description = inquire::Editor::new(edit_prompt_for("a description").as_str())
            .with_predefined_text("Enter a repository description")
            .prompt()?;
        args.description.replace(new_description);
    }

    if selected_options.contains(&Private) {
        let selected_privacy =
            fuzzy_select_with_key(Privacy::iter().collect(), select_prompt_for("visibility"))?
                .ok_or_else(|| anyhow::anyhow!("Nothing selected even though it was required"))?;

        args.private.replace(selected_privacy);
    }

    Ok(args)
}

fn create_body(args: RepoCreateArgs) -> CreateRepoOption {
    let RepoCreateArgs {
        default_branch,
        description,
        name,
        private,
    } = args;
    let mut options = CreateRepoOption::new(name.unwrap_or_default());

    if let Some(default_branch) = default_branch {
        options = options.with_default_branch(default_branch);
    }

    if let Some(description) = description {
        options = options.with_description(description);
    }

    match private {
        Some(Privacy::Public) => {
            options = options.public();
        }
        Some(Privacy::Private) => {
            options = options.private();
        }
        None => {}
    }
    options
}
