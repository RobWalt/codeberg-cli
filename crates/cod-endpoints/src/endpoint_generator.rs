use std::str::FromStr;

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

impl EndpointGenerator {
    fn repos_owner_repo(owner: impl ToString, repo: impl ToString) -> anyhow::Result<Url> {
        use crate::api::REPO_OWNER_REPOS;
        let url = Url::from_str(CODEBERG_API_BASE)?
            .join((REPO_OWNER_REPOS.to_owned() + "/").as_str())?
            .join((owner.to_string() + "/").as_str())?
            .join((repo.to_string() + "/").as_str())?;
        Ok(url)
    }

    pub fn repo_issues(owner: impl ToString, repo: impl ToString) -> anyhow::Result<Url> {
        use crate::api::REPO_ISSUES;
        let url = Self::repos_owner_repo(owner, repo)?.join(REPO_ISSUES)?;
        Ok(url)
    }

    pub fn repo_pulls(owner: impl ToString, repo: impl ToString) -> anyhow::Result<Url> {
        use crate::api::REPO_PULLS;
        let url = Self::repos_owner_repo(owner, repo)?.join(REPO_PULLS)?;
        Ok(url)
    }

    pub fn repo_labels(owner: impl ToString, repo: impl ToString) -> anyhow::Result<Url> {
        use crate::api::REPO_LABELS;
        let url = Self::repos_owner_repo(owner, repo)?.join(REPO_LABELS)?;
        Ok(url)
    }
}
