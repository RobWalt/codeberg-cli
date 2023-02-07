use cod_cli::pull_request::comment::CommentPullRequestArgs;
use cod_client::CodebergClient;
use cod_render::spinner::spin_until_ready;
use cod_render::ui::fuzzy_select_with_key;
use cod_types::api::create_options::create_comment_option::CreateCommentOption;
use cod_types::api::pull_request::PullRequest;

use crate::text_manipulation::select_prompt_for;

pub async fn comment_pull(
    _args: CommentPullRequestArgs,
    client: &CodebergClient,
) -> anyhow::Result<()> {
    let pull_requests_list = spin_until_ready(client.get_repo_prs(None, None)).await?;

    let selected_pull_request = fuzzy_select_with_key(
        pull_requests_list,
        select_prompt_for("pull request"),
        |pull_request: &PullRequest| format!("#{} {}", pull_request.number, pull_request.title),
        |pull_request| pull_request,
    )?;

    if let Some(pull_request) = selected_pull_request {
        let body = get_comment_input(pull_request.title.as_str())?;

        let comment = client
            .post_comment_for_id(pull_request.number, body)
            .await?;

        println!("Posted comment: {comment:?}");
    }
    Ok(())
}

fn get_comment_input(pull_request_title: &str) -> anyhow::Result<CreateCommentOption> {
    dialoguer::Editor::new()
        .edit(
            format!(
                "Write a comment for pull_request \"{}\"",
                pull_request_title
            )
            .as_str(),
        )?
        .map(CreateCommentOption::new)
        .ok_or_else(|| anyhow::anyhow!("Aborted submitting a comment."))
}
