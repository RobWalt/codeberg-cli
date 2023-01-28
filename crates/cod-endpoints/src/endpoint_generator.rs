use std::str::FromStr;

use cod_git_info::repo_owner::{get_repo_owner, RepoAndOwner};
use reqwest::Url;

use crate::api::CODEBERG_API_BASE;

#[derive(Debug, Clone, Copy)]
pub struct EndpointGenerator;

macro_rules! generator_method {
    ($method_name:ident, $endpoint:ident) => {
        impl EndpointGenerator {
            pub fn $method_name() -> anyhow::Result<Url> {
                use crate::api::$endpoint;
                let url = Url::from_str(CODEBERG_API_BASE)?.join($endpoint)?;
                Ok(url)
            }
        }
    };
}

generator_method!(verify, AUTHENTIFICATION_VERIFICATION);
generator_method!(user_info, USER_INFO);
generator_method!(user_followers, USER_FOLLOWERS);
generator_method!(user_following, USER_FOLLOWING);
generator_method!(user_repos, USER_REPOS);
generator_method!(user_search, USER_SEARCH);
generator_method!(repo_search, REPO_SEARCH);

impl EndpointGenerator {
    fn repos_owner_repo(endpoint: impl ToString) -> anyhow::Result<Url> {
        use crate::api::REPO_OWNER_REPOS;
        let RepoAndOwner { repo, owner } = get_repo_owner()?;
        let url = Url::from_str(CODEBERG_API_BASE)?
            .join((REPO_OWNER_REPOS.to_owned() + "/").as_str())?
            .join((owner + "/").as_str())?
            .join((repo + "/").as_str())?
            .join((endpoint.to_string()).as_str())?;
        Ok(url)
    }

    pub fn repo_assignees() -> anyhow::Result<Url> {
        use crate::api::REPO_ASSIGNEES;
        Self::repos_owner_repo(REPO_ASSIGNEES)
    }

    pub fn repo_infos() -> anyhow::Result<Url> {
        Self::repos_owner_repo("")
    }

    pub fn repo_issues() -> anyhow::Result<Url> {
        use crate::api::REPO_ISSUES;
        Self::repos_owner_repo(REPO_ISSUES)
    }

    pub fn repo_update_issue(issue_idx: usize) -> anyhow::Result<Url> {
        use crate::api::REPO_ISSUES;
        Self::repos_owner_repo(format!("{REPO_ISSUES}/{issue_idx}"))
    }

    pub fn repo_pull_requests() -> anyhow::Result<Url> {
        use crate::api::REPO_PULLS;
        Self::repos_owner_repo(REPO_PULLS)
    }

    pub fn repo_labels() -> anyhow::Result<Url> {
        use crate::api::REPO_LABELS;
        Self::repos_owner_repo(REPO_LABELS)
    }

    pub fn repo_forks(ownername: &str, reponame: &str) -> anyhow::Result<Url> {
        use crate::api::REPO_FORK;
        use crate::api::REPO_OWNER_REPOS;
        let url = Url::from_str(CODEBERG_API_BASE)?
            .join((REPO_OWNER_REPOS.to_owned() + "/").as_str())?
            .join(format!("{ownername}/").as_str())?
            .join(format!("{reponame}/").as_str())?
            .join(REPO_FORK)?;
        Ok(url)
    }

    pub fn repo_comments_for_id(issue_or_pr_id: usize) -> anyhow::Result<Url> {
        use crate::api::REPO_ISSUES;
        use crate::api::REPO_ISSUES_COMMENTS;
        Self::repos_owner_repo(format!("{REPO_ISSUES}/"))?
            .join(format!("{issue_or_pr_id}/").as_str())?
            .join(REPO_ISSUES_COMMENTS)
            .map_err(anyhow::Error::from)
    }

    pub fn repo_labels_with_id(label_id: usize) -> anyhow::Result<Url> {
        use crate::api::REPO_LABELS;
        Self::repos_owner_repo(format!("{REPO_LABELS}/"))?
            .join(format!("{label_id}").as_str())
            .map_err(anyhow::Error::from)
    }

    pub fn repo_branches() -> anyhow::Result<Url> {
        use crate::api::REPO_BRANCHES;
        Self::repos_owner_repo(REPO_BRANCHES)
    }

    pub fn repo_update_pull_request(pull_request_id: usize) -> anyhow::Result<Url> {
        use crate::api::REPO_PULLS;
        Self::repos_owner_repo(format!("{REPO_PULLS}/{pull_request_id}"))
    }
}
