use cod_cli::milestone::view::ViewMilestonesArgs;
use cod_client::CodebergClient;
use cod_render::spinner::spin_until_ready;
use cod_render::ui::fuzzy_select_with_key;
use cod_types::api::milestone::Milestone;
use cod_types::api::state_type::StateType;

use crate::text_manipulation::select_prompt_for;

pub async fn view_milestone(
    args: ViewMilestonesArgs,
    client: &CodebergClient,
) -> anyhow::Result<()> {
    let milestones_list =
        spin_until_ready(client.get_repo_milestones(Some(args.state), None)).await?;

    let selected_milestone = fuzzy_select_with_key(
        milestones_list,
        select_prompt_for("milestone"),
        |milestone: &Milestone| milestone.title.to_owned(),
        |milestone| milestone,
    )?
    .ok_or_else(|| anyhow::anyhow!("No milestone chosen. Aborting."))?;

    present_milestone_overview(selected_milestone, client, args.state).await?;

    Ok(())
}

async fn present_milestone_overview(
    milestone: Milestone,
    client: &CodebergClient,
    state: StateType,
) -> anyhow::Result<()> {
    use cod_render::prelude::*;
    use std::iter::once;

    let issues_list = spin_until_ready(client.get_repo_issues(Some(state), None)).await?;

    let mut milestone_issues = issues_list
        .iter()
        .filter(|&issue| {
            issue
                .milestone
                .as_ref()
                .map_or(false, |issue_milestone| issue_milestone.id == milestone.id)
        })
        .map(|issue| {
            format!(
                "#{}{}",
                issue.number,
                if issue.state.is_done() {
                    "✓ "
                } else {
                    "○ "
                }
            )
        })
        .collect::<Vec<_>>();

    milestone_issues.sort();

    let rows = once([
        TableCell::new_with_alignment("Name", 1, Alignment::Center),
        TableCell::new_with_alignment(milestone.title, 1, Alignment::Left),
    ])
    .chain(once([
        TableCell::new_with_alignment("Status", 1, Alignment::Center),
        TableCell::new_with_alignment(milestone.state, 1, Alignment::Left),
    ]))
    .chain(milestone.description.iter().map(|description| {
        [
            TableCell::new_with_alignment("Description", 1, Alignment::Center),
            TableCell::new_with_alignment(description, 1, Alignment::Left),
        ]
    }))
    .chain(once([
        TableCell::new_with_alignment("Related issues", 1, Alignment::Center),
        TableCell::new_with_alignment(milestone_issues.join(", "), 1, Alignment::Left),
    ]))
    .chain(once([
        TableCell::new_with_alignment("Progress", 1, Alignment::Center),
        TableCell::new_with_alignment(
            format!(
                "Progress: {} / {} done",
                milestone.closed_issues,
                milestone.open_issues + milestone.closed_issues
            ),
            1,
            Alignment::Left,
        ),
    ]))
    .chain(milestone.due_on.iter().map(|due_on| {
        [
            TableCell::new_with_alignment("Due on", 1, Alignment::Center),
            TableCell::new_with_alignment(due_on, 1, Alignment::Left),
        ]
    }))
    .map(Row::new);

    let table = CodTableBuilder::new().add_rows(rows).build();

    println!("{}", table.render());

    Ok(())
}
