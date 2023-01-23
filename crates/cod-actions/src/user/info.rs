use cod_cli::user::info::InfoArgs;
use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_render::spinner::spin_until_ready;
use cod_types::api::repo_info::RepoInfo;
use cod_types::api::user_info::UserInfo;
use cod_types::client::CodebergClient;
use cod_types::token::Token;

struct UserData {
    username: String,
    following_count: usize,
    followers_count: usize,
    repos_count: usize,
    top_repos: Vec<RepoInfo>,
}

pub async fn info(_args: InfoArgs, token: Token) -> anyhow::Result<()> {
    let client = CodebergClient::new(&token)?;

    let user_data = spin_until_ready(get_user_data(&client)).await?;

    present_user_info(user_data);

    Ok(())
}

async fn get_user_data(client: &CodebergClient) -> anyhow::Result<UserData> {
    let user_info = get_user_info(client).await?;
    let repos_info = get_repos_info(client).await?;
    let repos_count = repos_info.len();
    let top_repos = get_top_n_repos(repos_info, 5);
    Ok(UserData {
        username: user_info.username,
        followers_count: user_info.followers_count,
        following_count: user_info.following_count,
        repos_count,
        top_repos,
    })
}

async fn get_user_info(client: &CodebergClient) -> anyhow::Result<UserInfo> {
    let api_endpoint = EndpointGenerator::user_info()?;

    let user_info = client.get::<UserInfo>(api_endpoint).await?;

    Ok(user_info)
}

async fn get_repos_info(client: &CodebergClient) -> anyhow::Result<Vec<RepoInfo>> {
    let api_endpoint = EndpointGenerator::user_repos()?;

    let repos_info = client.get::<Vec<RepoInfo>>(api_endpoint).await?;

    Ok(repos_info)
}

fn get_top_n_repos(mut repos_info: Vec<RepoInfo>, n: usize) -> Vec<RepoInfo> {
    repos_info.sort_by_key(|repo| -repo.stars_count);
    repos_info.into_iter().take(n).collect::<Vec<_>>()
}

fn present_user_info(user_data: UserData) {
    use cod_render::prelude::*;

    let rows = [
        Row::new([
            TableCell::new_with_alignment("Username", 1, Alignment::Left),
            TableCell::new_with_alignment(user_data.username, 1, Alignment::Center),
        ]),
        Row::new([
            TableCell::new_with_alignment("Followers", 1, Alignment::Left),
            TableCell::new_with_alignment(user_data.followers_count, 1, Alignment::Center),
        ]),
        Row::new([
            TableCell::new_with_alignment("Following", 1, Alignment::Left),
            TableCell::new_with_alignment(user_data.following_count, 1, Alignment::Center),
        ]),
        Row::new([
            TableCell::new_with_alignment("Repos", 1, Alignment::Left),
            TableCell::new_with_alignment(user_data.repos_count, 1, Alignment::Center),
        ]),
        Row::new([
            TableCell::new_with_alignment("Top Repos", 1, Alignment::Left),
            TableCell::new_with_alignment(
                user_data
                    .top_repos
                    .into_iter()
                    .map(|repo| format!("- {} ({}⭐)", repo.name, repo.stars_count))
                    .collect::<Vec<_>>()
                    .join("\n"),
                1,
                Alignment::Left,
            ),
        ]),
    ];

    let table = CodTableBuilder::new().add_rows(rows).build();

    println!("{}", table.render());
}
