use cod_cli::issue::comment::CommentIssueArgs;
use cod_client::CodebergClient;
use cod_render::spinner::spin_until_ready;
use cod_render::ui::fuzzy_select_with_key;
use cod_types::api::create_options::create_comment_option::CreateCommentOption;

use crate::text_manipulation::select_prompt_for;

pub async fn comment_issue(_args: CommentIssueArgs, client: &CodebergClient) -> anyhow::Result<()> {
    let issues_list = spin_until_ready(client.get_repo_issues(None, None)).await?;

    let selected_issue = fuzzy_select_with_key(issues_list, select_prompt_for("issue"))?;

    if let Some(issue) = selected_issue {
        let body = get_comment_input(issue.title.as_str())?;

        let comment = client.post_comment_for_id(issue.number, body).await?;

        println!("Posted comment: {comment:?}");
    }
    Ok(())
}

fn get_comment_input(issue_title: &str) -> anyhow::Result<CreateCommentOption> {
    dialoguer::Editor::new()
        .edit(format!("Write a comment for issue \"{}\"", issue_title).as_str())?
        .map(CreateCommentOption::new)
        .ok_or_else(|| anyhow::anyhow!("Aborted submitting a comment."))
}
