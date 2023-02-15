use std::fmt::Display;
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
                Url::from_str(CODEBERG_API_BASE)?
                    .join($endpoint)
                    .map_err(anyhow::Error::from)
            }
        }
    };
}

macro_rules! generator_owner_repo_method {
    ($method_name:ident, $endpoint:ident) => {
        impl EndpointGenerator {
            pub fn $method_name() -> anyhow::Result<Url> {
                use crate::api::$endpoint;
                Self::repos_owner_repo($endpoint)
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
generator_method!(all_notifications, NOTIFICATIONS);

generator_owner_repo_method!(repo_infos, REPO_INFOS);
generator_owner_repo_method!(repo_assignees, REPO_ASSIGNEES);
generator_owner_repo_method!(repo_issues, REPO_ISSUES);
generator_owner_repo_method!(repo_pull_requests, REPO_PULLS);
generator_owner_repo_method!(repo_labels, REPO_LABELS);
generator_owner_repo_method!(repo_milestones, REPO_MILESTONES);
generator_owner_repo_method!(repo_branches, REPO_BRANCHES);

impl EndpointGenerator {
    fn repos_owner_repo(endpoint: impl Display) -> anyhow::Result<Url> {
        use crate::api::REPO_OWNER_REPOS;
        let RepoAndOwner { repo, owner } = get_repo_owner()?;
        Url::from_str(CODEBERG_API_BASE)?
            .join(format!("{REPO_OWNER_REPOS}/{owner}/{repo}/{endpoint}").as_str())
            .map_err(anyhow::Error::from)
    }

    pub fn repo_update_issue(issue_id: usize) -> anyhow::Result<Url> {
        use crate::api::REPO_ISSUES;
        Self::repos_owner_repo(format!("{REPO_ISSUES}/{issue_id}"))
    }

    pub fn repo_put_issue_labels(issue_id: usize) -> anyhow::Result<Url> {
        use crate::api::REPO_ISSUES;
        use crate::api::REPO_LABELS;
        Self::repos_owner_repo(format!("{REPO_ISSUES}/{issue_id}/{REPO_LABELS}"))
    }

    pub fn repo_update_milestone(milestone_id: usize) -> anyhow::Result<Url> {
        use crate::api::REPO_MILESTONES;
        Self::repos_owner_repo(format!("{REPO_MILESTONES}/{milestone_id}"))
    }

    pub fn repo_forks(ownername: &str, reponame: &str) -> anyhow::Result<Url> {
        use crate::api::REPO_FORK;
        Self::repos_owner_repo(format!("{ownername}/{reponame}/{REPO_FORK}"))
    }

    pub fn repo_comments_for_id(issue_or_pr_id: usize) -> anyhow::Result<Url> {
        use crate::api::REPO_ISSUES;
        use crate::api::REPO_ISSUES_COMMENTS;
        Self::repos_owner_repo(format!(
            "{REPO_ISSUES}/{issue_or_pr_id}/{REPO_ISSUES_COMMENTS}"
        ))
    }

    pub fn repo_labels_with_id(label_id: usize) -> anyhow::Result<Url> {
        use crate::api::REPO_LABELS;
        Self::repos_owner_repo(format!("{REPO_LABELS}/{label_id}"))
    }

    pub fn repo_update_pull_request(pull_request_id: usize) -> anyhow::Result<Url> {
        use crate::api::REPO_PULLS;
        Self::repos_owner_repo(format!("{REPO_PULLS}/{pull_request_id}"))
    }

    pub fn get_user_repos(username: String) -> anyhow::Result<Url> {
        use crate::api::USERS_BASE;
        Url::from_str(CODEBERG_API_BASE)?
            .join(format!("{USERS_BASE}/{username}/repos").as_str())
            .map_err(anyhow::Error::from)
    }

    pub fn get_org_repos(orgname: String) -> anyhow::Result<Url> {
        use crate::api::ORG_BASE;
        Url::from_str(CODEBERG_API_BASE)?
            .join(format!("{ORG_BASE}/{orgname}/repos").as_str())
            .map_err(anyhow::Error::from)
    }

    pub fn get_notification_thread(thread_id: usize) -> anyhow::Result<Url> {
        use crate::api::NOTIFICATIONS_INFO;
        Url::from_str(CODEBERG_API_BASE)?
            .join(format!("{NOTIFICATIONS_INFO}/{thread_id}").as_str())
            .map_err(anyhow::Error::from)
    }
}
