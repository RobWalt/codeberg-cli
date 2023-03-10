use crate::endpoints::endpoint_generator::EndpointGenerator;
use crate::types::api::branch::Branch;
use crate::types::api::comment::Comment;
use crate::types::api::create_options::create_comment_option::CreateCommentOption;
use crate::types::api::create_options::create_fork_option::CreateForkOption;
use crate::types::api::issue::Issue;
use crate::types::api::issue_labels_option::IssueLabelsOption;
use crate::types::api::label::Label;
use crate::types::api::milestone::Milestone;
use crate::types::api::notification::notification_state_type::NotificationStateType;
use crate::types::api::notification::notification_thread::NotificationThread;
use crate::types::api::notification::notification_type::NotificationSubjectType;
use crate::types::api::pull_request::PullRequest;
use crate::types::api::repository::Repository;
use crate::types::api::search_results::SearchResults;
use crate::types::api::state_type::StateType;
use crate::types::api::user::User;
use chrono::DateTime;
use chrono::Utc;
use reqwest::Url;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

use crate::client::BergClient;

impl BergClient {
    pub async fn get_user_info(&self) -> anyhow::Result<User> {
        let api = EndpointGenerator::user_info()?;
        self.get(api).await
    }

    pub async fn get_all_repos_info(&self) -> anyhow::Result<Vec<Repository>> {
        let api = EndpointGenerator::user_repos()?;
        self.get(api).await
    }

    pub async fn get_repo_info(&self) -> anyhow::Result<Repository> {
        let api = EndpointGenerator::repo_infos()?;
        self.get(api).await
    }

    pub async fn get_repo_labels(&self, maybe_limit: Option<usize>) -> anyhow::Result<Vec<Label>> {
        let api = EndpointGenerator::repo_labels()?;
        if let Some(limit) = maybe_limit {
            self.get_query(api, [("limit", limit)]).await
        } else {
            self.get(api).await
        }
    }

    pub async fn get_state_limit_list<T>(
        &self,
        maybe_state: Option<StateType>,
        maybe_limit: Option<usize>,
        api: Url,
    ) -> anyhow::Result<T>
    where
        T: DeserializeOwned + Debug,
    {
        use std::iter::once;

        let query_args = once((
            "limit",
            maybe_limit.map_or_else(|| usize::MAX.to_string(), |limit| limit.to_string()),
        ))
        .chain(once((
            "state",
            maybe_state.unwrap_or(StateType::All).to_string(),
        )))
        .collect::<Vec<_>>();
        if query_args.is_empty() {
            self.get(api).await
        } else {
            self.get_query(api, query_args).await
        }
    }

    pub async fn get_repo_milestones(
        &self,
        maybe_state: Option<StateType>,
        maybe_limit: Option<usize>,
    ) -> anyhow::Result<Vec<Milestone>> {
        let api = EndpointGenerator::repo_milestones()?;
        self.get_state_limit_list(maybe_state, maybe_limit, api)
            .await
    }

    pub async fn get_repo_issues(
        &self,
        maybe_state: Option<StateType>,
        maybe_limit: Option<usize>,
    ) -> anyhow::Result<Vec<Issue>> {
        let api = EndpointGenerator::repo_issues()?;
        self.get_state_limit_list(maybe_state, maybe_limit, api)
            .await
    }

    pub async fn get_repo_prs(
        &self,
        maybe_state: Option<StateType>,
        maybe_limit: Option<usize>,
    ) -> anyhow::Result<Vec<PullRequest>> {
        let api = EndpointGenerator::repo_pull_requests()?;
        self.get_state_limit_list(maybe_state, maybe_limit, api)
            .await
    }

    pub async fn get_repo_assignees(&self) -> anyhow::Result<Vec<User>> {
        let api = EndpointGenerator::repo_assignees()?;
        self.get(api).await
    }

    pub async fn search_for_user(
        &self,
        username: &str,
    ) -> anyhow::Result<SearchResults<Vec<User>>> {
        let api = EndpointGenerator::user_search()?;
        self.get_query(api, [("q", username)]).await
    }

    pub async fn search_for_repo(
        &self,
        reponame: &str,
        user_id: usize,
    ) -> anyhow::Result<SearchResults<Vec<Repository>>> {
        let api = EndpointGenerator::repo_search()?;
        self.get_query(
            api,
            [("q", reponame.to_string()), ("uid", user_id.to_string())],
        )
        .await
    }

    pub async fn fork_repo(&self, ownername: &str, reponame: &str) -> anyhow::Result<Repository> {
        let api = EndpointGenerator::repo_forks(ownername, reponame)?;
        let body = CreateForkOption::same_repo_name();
        self.post_query_body(api, body, [("owner", ownername), ("repo", reponame)])
            .await
    }

    pub async fn get_comments_for_id(&self, issue_or_pr_id: usize) -> anyhow::Result<Vec<Comment>> {
        let api = EndpointGenerator::repo_comments_for_id(issue_or_pr_id)?;
        self.get(api).await
    }

    pub async fn post_comment_for_id(
        &self,
        issue_id: usize,
        comment: CreateCommentOption,
    ) -> anyhow::Result<Comment> {
        let api = EndpointGenerator::repo_comments_for_id(issue_id)?;
        self.post_body(api, comment).await
    }

    pub async fn delete_label(&self, label_id: usize) -> anyhow::Result<()> {
        let api = EndpointGenerator::repo_labels_with_id(label_id)?;
        self.delete(api).await
    }

    pub async fn get_repo_branches(&self) -> anyhow::Result<Vec<Branch>> {
        let api = EndpointGenerator::repo_branches()?;
        self.get(api).await
    }

    pub async fn get_user_repos(&self, username: String) -> anyhow::Result<Vec<Repository>> {
        let api = EndpointGenerator::get_user_repos(username)?;
        self.get(api).await
    }

    pub async fn get_org_repos(&self, orgname: String) -> anyhow::Result<Vec<Repository>> {
        let api = EndpointGenerator::get_org_repos(orgname)?;
        self.get(api).await
    }

    pub async fn get_user_or_org_repos(&self, name: String) -> anyhow::Result<Vec<Repository>> {
        self.get_org_repos(name.clone())
            .await
            .or(self.get_user_repos(name).await)
    }

    pub async fn replace_labels(
        &self,
        issue_id: usize,
        issue_labels_option: IssueLabelsOption,
    ) -> anyhow::Result<Vec<Label>> {
        let api = EndpointGenerator::repo_put_issue_labels(issue_id)?;
        self.put_body(api, issue_labels_option).await
    }

    pub async fn get_all_notifications_unfiltered(
        &self,
        all: bool,
    ) -> anyhow::Result<Vec<NotificationThread>> {
        self.get_all_notifications_filtered(
            all,
            None,
            None,
            vec![NotificationStateType::Pinned, NotificationStateType::Unread],
            None,
            1,
            100000,
        )
        .await
    }

    pub async fn get_all_notifications_filtered(
        &self,
        all: bool,
        since: Option<DateTime<Utc>>,
        before: Option<DateTime<Utc>>,
        status_types: Vec<NotificationStateType>,
        subject_type: Option<NotificationSubjectType>,
        page: usize,
        limit: usize,
    ) -> anyhow::Result<Vec<NotificationThread>> {
        use std::iter::once;
        let api = EndpointGenerator::all_notifications()?;
        let query = since
            .iter()
            .map(|since| ("since", since.to_rfc3339()))
            .chain(before.iter().map(|before| ("before", before.to_rfc3339())))
            .chain(
                status_types
                    .iter()
                    .map(|status_type| ("status-types", status_type.to_string())),
            )
            .chain(
                subject_type
                    .iter()
                    .map(|subject_type| ("subject-type", subject_type.to_string())),
            )
            .chain(once(("page", page.to_string())))
            .chain(once(("limit", limit.to_string())))
            .chain(once(("all", all.to_string())))
            .collect::<Vec<_>>();
        self.get_query(api, query).await
    }

    pub async fn get_notification_thread(
        &self,
        thread_id: usize,
    ) -> anyhow::Result<NotificationThread> {
        let api = EndpointGenerator::get_notification_thread(thread_id)?;
        self.get(api).await
    }
}
