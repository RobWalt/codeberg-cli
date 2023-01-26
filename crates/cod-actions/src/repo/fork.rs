use cod_cli::repo::fork::RepoForkArgs;
use cod_client::CodebergClient;
use cod_render::spinner::spin_until_ready;

pub async fn fork_repo(args: RepoForkArgs, client: &CodebergClient) -> anyhow::Result<()> {
    let (ownername, reponame) = parse_owner_and_repo(args.owner_and_repo)?;
    let ssh_url = spin_until_ready(start_fork_repo(
        client,
        ownername.as_str(),
        reponame.as_str(),
    ))
    .await?;
    ask_confirm_clone(reponame.as_str())?;
    start_clone_repo(ssh_url)?;
    Ok(())
}

fn parse_owner_and_repo(owner_and_repo: String) -> anyhow::Result<(String, String)> {
    owner_and_repo
        .split_once('/')
        .map(|(owner, repo)| (owner.to_owned(), repo.to_owned()))
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Please provide the repository you want to fork in the format OWNER/REPO."
            )
        })
}

async fn start_fork_repo(
    client: &CodebergClient,
    ownername: &str,
    reponame: &str,
) -> anyhow::Result<String> {
    // just to check if the repo exists
    let _ssh_url_original = get_ssh_url(client, ownername, reponame).await?;
    client.fork_repo(ownername, reponame).await?;
    let user = client.get_user_info().await?;
    let new_url = get_ssh_url(client, user.username.as_str(), reponame).await?;
    Ok(new_url)
}

async fn get_ssh_url(
    client: &CodebergClient,
    ownername: &str,
    reponame: &str,
) -> anyhow::Result<String> {
    let user = client
        .search_for_user(ownername)
        .await
        .and_then(|matching_users| {
            matching_users
                .data
                .first()
                .ok_or_else(|| {
                    anyhow::anyhow!("Couldn't find repo when searching for {ownername}/{reponame}")
                })
                .cloned()
        })?;
    let repo = client
        .search_for_repo(reponame, user.id)
        .await
        .and_then(|matching_repos| {
            matching_repos
                .data
                .first()
                .ok_or_else(|| {
                    anyhow::anyhow!("Couldn't find repo when searching for {ownername}/{reponame}")
                })
                .cloned()
        })?;
    Ok(repo.ssh_url)
}

fn ask_confirm_clone(reponame: &str) -> anyhow::Result<()> {
    let current_path = std::env::current_dir()?;
    dialoguer::Confirm::new()
        .with_prompt(format!(
            "Do you really to fork {reponame} into the directory {current_path:?}"
        ))
        .interact()
        .map_err(anyhow::Error::from)
        .and_then(|confirmed| {
            confirmed
                .then_some(())
                .ok_or_else(|| anyhow::anyhow!("Abort cloning the repository."))
        })
}

fn start_clone_repo(ssh_url: String) -> anyhow::Result<()> {
    let mut cmd = std::process::Command::new("git");
    cmd.arg("clone").arg(ssh_url);
    tracing::info!("cmd: {cmd:?}");
    cmd.stdout(std::process::Stdio::inherit()).spawn()?;
    Ok(())
}
