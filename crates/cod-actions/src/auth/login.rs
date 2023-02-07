use std::path::{Path, PathBuf};

use anyhow::Context;
use cod_cli::auth::login::LoginArgs;
use cod_client::CodebergClient;
use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_paths::token_directory;
use cod_render::spinner::spin_until_ready;
use cod_types::api::user::User;
use cod_types::token::Token;
use inquire::validator::Validation;
use inquire::CustomUserError;

const TOKEN_GENERATION_URL: &str = "https://codeberg.org/user/settings/applications";

pub async fn login_user(_args: LoginArgs) -> anyhow::Result<()> {
    // ask for usage of browser
    if inquire::Confirm::new("Authenticating. Open Browser to generate token for codeberg-cli?")
        .prompt()?
    {
        println!("\nOpening Browser...\n");
        webbrowser::open(TOKEN_GENERATION_URL)?;
    } else {
        println!(
            "\nYou chose not to authenticate via browser. Visit\n\n\t{}\n\nto generate a token.\n",
            TOKEN_GENERATION_URL
        );
    }

    // get token from user
    let token = ask_for_token()?;

    // this is where the token gets stored
    let token_path = create_token_storage_path()?;

    // save the token
    std::fs::write(token_path.as_path(), token.as_str())?;

    spin_until_ready(async {
        verify_setup(&token)
            .await
            .or_else(cleanup_token_failed_verification(token_path.as_path()))
    })
    .await
}

fn cleanup_token_failed_verification(
    token_path: &Path,
) -> impl FnOnce(anyhow::Error) -> anyhow::Result<()> + '_ {
    move |error: anyhow::Error| match std::fs::remove_file(token_path) {
        Err(e) if matches!(e.kind(), std::io::ErrorKind::PermissionDenied) => {
            anyhow::bail!("Couldn't delete saved broken token. Make sure to delete it to prevent malicious use from potential attackers.");
        }
        _ => Err(error),
    }
}

async fn verify_setup(token: &Token) -> anyhow::Result<()> {
    let client = CodebergClient::new(token).context("Couldn't create CodClient.")?;

    let verification_api_endpoint = EndpointGenerator::verify()?;

    _ = client
        .get::<User>(verification_api_endpoint)
        .await
        .map_err(|_| {
            anyhow::anyhow!("Verification API call didn't contain expected information.")
        })?;

    println!("\nAuthentication success!");

    Ok(())
}

fn create_token_storage_path() -> anyhow::Result<PathBuf> {
    token_directory().and_then(|token_dir| {
        std::fs::create_dir_all(&token_dir)
            .context("Couldn't create directory for saving the token.")?;
        Ok(token_dir.join("TOKEN"))
    })
}

fn validate_token(input: &str) -> Result<Validation, CustomUserError> {
    let v = validate_word_count(input);
    if let Validation::Invalid(_) = v {
        return Ok(v);
    }
    Ok(validate_token_length(input))
}

fn validate_word_count(input: &str) -> Validation {
    let words = input.split_whitespace().collect::<Vec<_>>();
    if words.len() != 1 {
        Validation::Invalid(
            format!(
                "Token is just one word. Your input words were\n{}",
                words
                    .iter()
                    .map(|word| format!("  - {word}"))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
            .into(),
        )
    } else {
        Validation::Valid
    }
}

fn validate_token_length(token: &str) -> Validation {
    if token.len() != 40 {
        Validation::Invalid(
            format!(
                "Usual token length is 40. Token\n\n\t{token:?}\n\nhas length {}",
                token.len()
            )
            .into(),
        )
    } else {
        Validation::Valid
    }
}

fn ask_for_token() -> anyhow::Result<Token> {
    inquire::Text::new("Token")
        .with_validator(validate_token)
        .prompt()
        .map(Token::new)
        .map_err(anyhow::Error::from)
}
