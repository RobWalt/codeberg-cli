use crate::cli::milestone::view::ViewMilestonesArgs;
use crate::client::BergClient;
use crate::render::spinner::spin_until_ready;
use crate::render::ui::fuzzy_select_with_key;
use crate::types::api::milestone::Milestone;
use crate::types::api::state_type::StateType;

use crate::actions::text_manipulation::select_prompt_for;

pub async fn view_milestone(args: ViewMilestonesArgs, client: &BergClient) -> anyhow::Result<()> {
    let milestones_list =
        spin_until_ready(client.get_repo_milestones(Some(args.state), None)).await?;

    let selected_milestone =
        fuzzy_select_with_key(milestones_list, select_prompt_for("milestone"))?
            .ok_or_else(|| anyhow::anyhow!("No milestone chosen. Aborting."))?;

    present_milestone_overview(selected_milestone, client, args.state).await?;

    Ok(())
}

async fn present_milestone_overview(
    milestone: Milestone,
    client: &BergClient,
    state: StateType,
) -> anyhow::Result<()> {
    use crate::render::table::builder::BergTableBuilder;
    use std::iter::once;
    use term_table::row::Row;
    use term_table::table_cell::{Alignment, TableCell};

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
    .chain(
        once(if milestone_issues.is_empty() {
            None
        } else {
            Some([
                TableCell::new_with_alignment("Related issues", 1, Alignment::Center),
                TableCell::new_with_alignment(milestone_issues.join(", "), 1, Alignment::Left),
            ])
        })
        .flatten(),
    )
    .chain(milestone.due_on.iter().map(|due_date| {
        [
            TableCell::new_with_alignment("Due on", 1, Alignment::Center),
            TableCell::new_with_alignment(
                due_date.format("%d. %b %Y - %H:%M").to_string(),
                1,
                Alignment::Left,
            ),
        ]
    }))
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

    let table = BergTableBuilder::new().add_rows(rows).build();

    println!("{}", table.render());

    Ok(())
}
