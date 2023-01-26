use cod_endpoints::endpoint_generator::EndpointGenerator;
use cod_types::api::comment::Comment;
use cod_types::api::create_fork_option::CreateForkOption;
use cod_types::api::issue::Issue;
use cod_types::api::label::Label;
use cod_types::api::pull_request::PullRequest;
use cod_types::api::repository::Repository;
use cod_types::api::search_results::SearchResults;
use cod_types::api::state_type::StateType;
use cod_types::api::user::User;

use crate::CodebergClient;

impl CodebergClient {
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

    pub async fn get_repo_prs(
        &self,
        maybe_limit: Option<usize>,
    ) -> anyhow::Result<Vec<PullRequest>> {
        let api = EndpointGenerator::repo_pull_requests()?;
        if let Some(limit) = maybe_limit {
            self.get_query(api, [("limit", limit)]).await
        } else {
            self.get(api).await
        }
    }

    pub async fn get_repo_labels(&self, maybe_limit: Option<usize>) -> anyhow::Result<Vec<Label>> {
        let api = EndpointGenerator::repo_labels()?;
        if let Some(limit) = maybe_limit {
            self.get_query(api, [("limit", limit)]).await
        } else {
            self.get(api).await
        }
    }

    pub async fn get_repo_issues(
        &self,
        maybe_state: Option<StateType>,
        maybe_limit: Option<usize>,
    ) -> anyhow::Result<Vec<Issue>> {
        let api = EndpointGenerator::repo_issues()?;
        let query_args = maybe_limit
            .map(|limit| ("limit", limit.to_string()))
            .into_iter()
            .chain(
                maybe_state
                    .map(|state| ("state", state.to_string()))
                    .into_iter(),
            )
            .collect::<Vec<_>>();
        if query_args.is_empty() {
            self.get(api).await
        } else {
            self.get_query(api, query_args).await
        }
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

    pub async fn get_comments_for_issue(&self, issue_id: usize) -> anyhow::Result<Vec<Comment>> {
        let api = EndpointGenerator::repo_comments_for_issue(issue_id)?;
        self.get(api).await
    }
}
