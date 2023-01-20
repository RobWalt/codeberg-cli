use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::Context;
use reqwest::Url;

use crate::endpoints::{AUTHENTIFICATION_VERIFICATION, CODEBERG_API_BASE};
use crate::paths::token_directory;
use crate::types::client::CodebergClient;
use crate::{LoginArgs, Token};

const TOKEN_GENERATION_URL: &str = "https://codeberg.org/user/settings/applications";

pub async fn login(_args: LoginArgs) -> anyhow::Result<()> {
    // ask for usage of browser
    if dialoguer::Confirm::new()
        .with_prompt("Authenticating. Open Browser to generate token for codeberg-cli?")
        .interact()?
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

    verify_setup(&token)
        .await
        .or_else(cleanup_token_failed_verification(token_path.as_path()))
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

    let verification_api_endpoint =
        Url::from_str(CODEBERG_API_BASE)?.join(AUTHENTIFICATION_VERIFICATION)?;

    let json_response = client.get(verification_api_endpoint).await?;

    match json_response {
        serde_json::Value::Object(map) if map.contains_key("username") => {}
        _ => anyhow::bail!("Verification API call didn't contain expected information."),
    };

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

fn validate_token(input: &String) -> anyhow::Result<()> {
    validate_word_count(input.as_str()).and_then(validate_token_length)
}

fn validate_word_count(input: &str) -> anyhow::Result<&str> {
    let words = input.split_whitespace().collect::<Vec<_>>();
    if words.len() != 1 {
        anyhow::bail!(
            "Token is just one word. Your input words were\n{}",
            words
                .iter()
                .map(|word| format!("  - {word}"))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
    Ok(words[0])
}

fn validate_token_length(token: &str) -> anyhow::Result<()> {
    if token.len() != 40 {
        anyhow::bail!(
            "Usual token length is 40. Token\n\n\t{token:?}\n\nhas length {}",
            token.len()
        );
    }
    Ok(())
}

fn ask_for_token() -> anyhow::Result<Token> {
    dialoguer::Input::new()
        .with_prompt("Token")
        .allow_empty(false)
        .validate_with(validate_token)
        .interact()
        .map(Token)
        .map_err(anyhow::Error::from)
}
