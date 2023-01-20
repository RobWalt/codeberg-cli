use std::str::FromStr;

use anyhow::Context;
use reqwest::Url;

use crate::endpoints::{CODEBERG_API_BASE, USER_FOLLOWERS, USER_FOLLOWING, USER_INFO, USER_REPOS};
use crate::frontend::user::info::InfoArgs;
use crate::types::client::CodebergClient;
use crate::types::token::Token;

pub async fn info(args: InfoArgs, token: Token) -> anyhow::Result<()> {
    let client = CodebergClient::new(&token)?;

    let username = get_username(&client).await?;
    let followers_count = get_num_followers(&client).await?;
    let following_count = get_num_following(&client).await?;
    let repos_count = get_num_repos(&client).await?;
    let top_repos = get_top_n_repos(&client, 5).await?;

    present_user_info(
        username,
        followers_count,
        following_count,
        repos_count,
        top_repos,
    );

    Ok(())
}

async fn get_username(client: &CodebergClient) -> anyhow::Result<String> {
    let api_endpoint = Url::from_str(CODEBERG_API_BASE)?.join(USER_INFO)?;

    let user_info = client.get(api_endpoint).await?;

    user_info
        .get("username")
        .and_then(|username| username.as_str())
        .map(|username| username.to_owned())
        .context("Couldn't find user name.")
}

async fn get_num_followers(client: &CodebergClient) -> anyhow::Result<usize> {
    let api_endpoint = Url::from_str(CODEBERG_API_BASE)?.join(USER_FOLLOWERS)?;

    let followers_info = client.get(api_endpoint).await?;

    followers_info
        .as_array()
        .context("Expected array of followers.")
        .map(|array| array.len())
}

async fn get_num_following(client: &CodebergClient) -> anyhow::Result<usize> {
    let api_endpoint = Url::from_str(CODEBERG_API_BASE)?.join(USER_FOLLOWING)?;

    let following_info = client.get(api_endpoint).await?;

    following_info
        .as_array()
        .context("Expected array of following.")
        .map(|array| array.len())
}

async fn get_num_repos(client: &CodebergClient) -> anyhow::Result<usize> {
    let api_endpoint = Url::from_str(CODEBERG_API_BASE)?.join(USER_REPOS)?;

    let repos_info = client.get(api_endpoint).await?;

    repos_info
        .as_array()
        .context("Expected array of repos.")
        .map(|array| array.len())
}

async fn get_top_n_repos(client: &CodebergClient, n: usize) -> anyhow::Result<Vec<String>> {
    let api_endpoint = Url::from_str(CODEBERG_API_BASE)?.join(USER_REPOS)?;

    let repos_info = client.get(api_endpoint).await?;

    repos_info
        .as_array()
        .cloned()
        .context("Expected array of repos.")
        .map(|mut array| {
            // sort by popularity
            array.sort_by_key(|repo| {
                -repo
                    .get("stars_count")
                    .and_then(|stars| stars.as_i64())
                    .unwrap_or_default()
            });
            // return names
            array
                .into_iter()
                .filter_map(|repo| {
                    repo.get("name")
                        .and_then(|name| name.as_str())
                        .map(|name| name.to_owned())
                })
                .take(n)
                .collect()
        })
}

fn present_user_info(
    username: String,
    followers_count: usize,
    following_count: usize,
    repos_count: usize,
    top_repos: Vec<String>,
) {
    use term_table::row::Row;
    use term_table::table_cell::Alignment;
    use term_table::table_cell::TableCell;
    use term_table::{Table, TableStyle};

    let mut table = Table::new();
    table.max_column_width(40);
    table.style = TableStyle::elegant();

    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("Username", 1, Alignment::Left),
        TableCell::new_with_alignment(username, 1, Alignment::Center),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("Followers", 1, Alignment::Left),
        TableCell::new_with_alignment(followers_count, 1, Alignment::Center),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("Following", 1, Alignment::Left),
        TableCell::new_with_alignment(following_count, 1, Alignment::Center),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("Repos", 1, Alignment::Left),
        TableCell::new_with_alignment(repos_count, 1, Alignment::Center),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("Top Repos", 1, Alignment::Left),
        TableCell::new_with_alignment(top_repos.join(", "), 1, Alignment::Center),
    ]));

    println!("{}", table.render());
}
