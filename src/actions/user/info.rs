use std::str::FromStr;

use reqwest::Url;

use crate::endpoints::{CODEBERG_API_BASE, USER_INFO};
use crate::frontend::user::info::InfoArgs;
use crate::types::client::CodebergClient;
use crate::types::response::JSONResponse;
use crate::types::token::Token;

pub async fn info(args: InfoArgs, token: Token) -> anyhow::Result<()> {
    let client = CodebergClient::new(&token)?;

    let api_endpoint = Url::from_str(CODEBERG_API_BASE)?.join(USER_INFO)?;

    let user_info = client
        .get(api_endpoint)
        .send()
        .await?
        .json::<JSONResponse>()
        .await?;

    println!("{user_info:#?}");

    Ok(())
}
