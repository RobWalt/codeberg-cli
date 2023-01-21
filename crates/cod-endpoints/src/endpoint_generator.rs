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
    pub fn issues_list(owner: impl ToString, repo: impl ToString) -> anyhow::Result<Url> {
        use crate::api::{ISSUE_LIST_END, ISSUE_LIST_START};
        let url = Url::from_str(CODEBERG_API_BASE)?
            .join((ISSUE_LIST_START.to_owned() + "/").as_str())?
            .join((owner.to_string() + "/").as_str())?
            .join((repo.to_string() + "/").as_str())?
            .join(ISSUE_LIST_END)?;
        Ok(url)
    }

    pub fn pull_list(owner: impl ToString, repo: impl ToString) -> anyhow::Result<Url> {
        use crate::api::{PULL_LIST_END, PULL_LIST_START};
        let url = Url::from_str(CODEBERG_API_BASE)?
            .join((PULL_LIST_START.to_owned() + "/").as_str())?
            .join((owner.to_string() + "/").as_str())?
            .join((repo.to_string() + "/").as_str())?
            .join(PULL_LIST_END)?;
        Ok(url)
    }
}
