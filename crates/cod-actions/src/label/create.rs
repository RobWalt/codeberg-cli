use cod_cli::label::create::CreateLabelArgs;
use cod_client::CodebergClient;
use cod_render::ui::multi_fuzzy_select_with_key;
use cod_types::api::create_options::create_label_options::CreateLabelOption;

use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_types::api::label::Label;
use inquire::validator::Validation;
use strum::Display;

use crate::text_manipulation::edit_prompt_for;

pub async fn create_label(args: CreateLabelArgs, client: &CodebergClient) -> anyhow::Result<()> {
    let options = fill_in_mandatory_values(&args)?;
    let options = fill_in_optional_values(options, args)?;

    let api_endpoint = EndpointGenerator::repo_labels()?;

    let label: Label = client.post_body(api_endpoint, options).await?;

    println!("Successfully created label: {}", label.name);

    Ok(())
}

fn fill_in_mandatory_values(args: &CreateLabelArgs) -> anyhow::Result<CreateLabelOption> {
    let name = match args.name.clone() {
        Some(name) => name,
        None => inquire::Text::new("Label Title").prompt()?,
    };
    Ok(CreateLabelOption::new(name))
}

fn fill_in_optional_values(
    mut options: CreateLabelOption,
    args: CreateLabelArgs,
) -> anyhow::Result<CreateLabelOption> {
    options = options
        .with_color(args.color.clone().unwrap_or_default())
        .with_description(args.description.clone().unwrap_or_default());

    #[derive(Debug, Clone, Copy, Display, PartialEq, Eq)]
    enum PossiblyMissing {
        Description,
        Color,
    }

    use PossiblyMissing::*;

    let missing_options = [
        args.color.is_none().then_some(Color),
        args.description.is_none().then_some(Description),
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
        let new_description = inquire::Editor::new(edit_prompt_for("a label description").as_str())
            .with_predefined_text("Enter a label description")
            .prompt()?;
        options = options.with_description(new_description);
    }

    if selected_options.contains(&Color) {
        let new_color = inquire::Text::new("Enter a color")
            .with_validator(|color: &str| {
                Ok((color.len() == 7
                    && color.starts_with('#')
                    && color
                        .chars()
                        .skip(1)
                        .take(6)
                        .filter(|digit| digit.is_ascii_hexdigit())
                        .count()
                        == 6)
                    .then_some(Validation::Valid)
                    .unwrap_or_else(|| Validation::Invalid("Not a color: format <#XXXXXX>".into())))
            })
            .prompt()?;

        options = options.with_color(new_color);
    }

    Ok(options)
}
