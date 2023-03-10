use crate::cli::repo::fork::RepoForkArgs;
use crate::client::BergClient;
use crate::render::spinner::{spin_and_try_every_second_for, spin_until_ready};

pub async fn fork_repo(args: RepoForkArgs, client: &BergClient) -> anyhow::Result<()> {
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
    client: &BergClient,
    ownername: &str,
    reponame: &str,
) -> anyhow::Result<String> {
    // just to check if the repo exists
    let ssh_url_original = get_ssh_url(client, ownername, reponame).await?;
    tracing::error!("Fork url: {ssh_url_original}");
    client.fork_repo(ownername, reponame).await?;
    tracing::debug!("Forked successfully.");
    let user = client.get_user_info().await?;
    tracing::debug!("User: {user:?}");
    let new_url =
        spin_and_try_every_second_for(|| get_ssh_url(client, user.username.as_str(), reponame), 10)
            .await?;
    tracing::debug!("Forked Repo SSH URL: {new_url:?}");
    Ok(new_url)
}

async fn get_ssh_url(
    client: &BergClient,
    ownername: &str,
    reponame: &str,
) -> anyhow::Result<String> {
    let owner_repos = client.get_user_or_org_repos(ownername.to_owned()).await?;
    owner_repos
        .iter()
        .find(|repo| repo.name == reponame)
        .ok_or_else(|| anyhow::anyhow!("User {ownername} doesn't own the repo {reponame}."))
        .map(|repo| repo.ssh_url.to_owned())
}

fn ask_confirm_clone(reponame: &str) -> anyhow::Result<()> {
    let current_path = std::env::current_dir()?;
    inquire::Confirm::new(
        format!("Do you really to fork {reponame} into the directory {current_path:?}").as_str(),
    )
    .prompt()
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
    tracing::debug!("cmd: {cmd:?}");
    cmd.stdout(std::process::Stdio::inherit()).spawn()?;
    Ok(())
}
