use cod_cli::user::info::UserInfoArgs;
use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_render::spinner::spin_until_ready;
use cod_types::api::repository::Repository;
use cod_types::api::user::User;
use cod_types::client::CodebergClient;
use cod_types::token::Token;

struct UserData {
    username: String,
    following_count: usize,
    followers_count: usize,
    repos_count: usize,
    top_repos: Vec<Repository>,
}

pub async fn user_info(_args: UserInfoArgs, token: Token) -> anyhow::Result<()> {
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

async fn get_user_info(client: &CodebergClient) -> anyhow::Result<User> {
    let api_endpoint = EndpointGenerator::user_info()?;

    let user_info = client.get::<User>(api_endpoint).await?;

    Ok(user_info)
}

async fn get_repos_info(client: &CodebergClient) -> anyhow::Result<Vec<Repository>> {
    let api_endpoint = EndpointGenerator::user_repos()?;

    let repos_info = client.get::<Vec<Repository>>(api_endpoint).await?;

    Ok(repos_info)
}

fn get_top_n_repos(mut repos_info: Vec<Repository>, n: usize) -> Vec<Repository> {
    repos_info.sort_by_key(|repo| -(repo.stars_count as isize));
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
