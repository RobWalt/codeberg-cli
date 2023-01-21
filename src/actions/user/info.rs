use crate::cli::user::info::InfoArgs;
use crate::endpoints::endpoint_generator::EndpointGenerator;
use crate::types::api::followers_info::FollowersInfo;
use crate::types::api::following_info::FollowingInfo;
use crate::types::api::repo_info::RepoInfo;
use crate::types::api::user_info::UserInfo;
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
    let api_endpoint = EndpointGenerator::user_info()?;

    let user_info = client.get::<UserInfo>(api_endpoint).await?;

    Ok(user_info.username)
}

async fn get_num_followers(client: &CodebergClient) -> anyhow::Result<usize> {
    let api_endpoint = EndpointGenerator::user_followers()?;

    let followers_info = client.get::<Vec<FollowersInfo>>(api_endpoint).await?;

    Ok(followers_info.len())
}

async fn get_num_following(client: &CodebergClient) -> anyhow::Result<usize> {
    let api_endpoint = EndpointGenerator::user_following()?;

    let following_info = client.get::<Vec<FollowingInfo>>(api_endpoint).await?;

    Ok(following_info.len())
}

async fn get_num_repos(client: &CodebergClient) -> anyhow::Result<usize> {
    let api_endpoint = EndpointGenerator::user_repos()?;

    let repos_info = client.get::<Vec<RepoInfo>>(api_endpoint).await?;

    Ok(repos_info.len())
}

async fn get_top_n_repos(client: &CodebergClient, n: usize) -> anyhow::Result<Vec<RepoInfo>> {
    let api_endpoint = EndpointGenerator::user_repos()?;

    let mut repos_info = client.get::<Vec<RepoInfo>>(api_endpoint).await?;

    repos_info.sort_by_key(|repo| -repo.stars_count);

    let top_repos = repos_info.into_iter().take(n).collect::<Vec<_>>();
    Ok(top_repos)
}

fn present_user_info(
    username: String,
    followers_count: usize,
    following_count: usize,
    repos_count: usize,
    top_repos: Vec<RepoInfo>,
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
        TableCell::new_with_alignment(
            top_repos
                .into_iter()
                .map(|repo| format!("- {} ({}⭐)", repo.name, repo.stars_count))
                .collect::<Vec<_>>()
                .join("\n"),
            1,
            Alignment::Left,
        ),
    ]));

    println!("{}", table.render());
}
