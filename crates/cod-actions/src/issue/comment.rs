use cod_cli::issue::comment::CommentIssueArgs;
use cod_client::CodebergClient;
use cod_render::spinner::spin_until_ready;
use cod_render::ui::fuzzy_select_with_key;
use cod_types::api::create_options::create_comment_option::CreateCommentOption;

use crate::text_manipulation::{edit_prompt_for, select_prompt_for};

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
    let new_comment = inquire::Editor::new(edit_prompt_for("a comment").as_str())
        .with_predefined_text(format!("Write a comment for issue \"{}\"", issue_title).as_str())
        .prompt()?;
    Ok(CreateCommentOption::new(new_comment))
}
