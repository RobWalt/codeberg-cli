use crate::cli::pull_request::comment::CommentPullRequestArgs;
use crate::client::BergClient;
use crate::render::spinner::spin_until_ready;
use crate::render::ui::fuzzy_select_with_key;
use crate::types::api::create_options::create_comment_option::CreateCommentOption;

use crate::actions::text_manipulation::{edit_prompt_for, select_prompt_for};

pub async fn comment_pull(
    _args: CommentPullRequestArgs,
    client: &BergClient,
) -> anyhow::Result<()> {
    let pull_requests_list = spin_until_ready(client.get_repo_prs(None, None)).await?;

    let selected_pull_request =
        fuzzy_select_with_key(pull_requests_list, select_prompt_for("pull request"))?;

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
    let comment = inquire::Editor::new(edit_prompt_for("a comment").as_str())
        .with_predefined_text(
            format!(
                "Write a comment for pull_request \"{}\"",
                pull_request_title
            )
            .as_str(),
        )
        .prompt()?;
    Ok(CreateCommentOption::new(comment))
}
