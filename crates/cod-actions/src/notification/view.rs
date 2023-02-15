use cod_cli::notification::view::ViewNotificationArgs;
use cod_client::CodebergClient;
use cod_render::ui::fuzzy_select_with_key;
use cod_types::api::notification::notification_thread::NotificationThread;

pub async fn view_notifications(
    args: ViewNotificationArgs,
    client: &CodebergClient,
) -> anyhow::Result<()> {
    let selected_notification_thread = if let Some(thread_id) = args.id {
        let notification_thread = client.get_notification_thread(thread_id).await?;
        tracing::debug!("{notification_thread:?}");
        notification_thread
    } else {
        let notification_threads_list = client.get_all_notifications().await?;

        let notification_thread =
            fuzzy_select_with_key(notification_threads_list, "notification thread")?
                .ok_or_else(|| anyhow::anyhow!("Nothing selected. Aborting."))?;
        tracing::debug!("{notification_thread:?}");
        notification_thread
    };

    present_notification_thread_details(selected_notification_thread);

    Ok(())
}

fn present_notification_thread_details(notification_thread: NotificationThread) {
    use cod_render::prelude::*;
    use std::iter::once;

    let reponame = {
        let repo = &notification_thread.repository;
        format!("{}/{}", repo.owner, repo.name)
    };

    let rows = once(Row::new([
        TableCell::new_with_alignment("Title", 1, Alignment::Center),
        TableCell::new_with_alignment(
            notification_thread.subject.title.as_str(),
            1,
            Alignment::Left,
        ),
    ]))
    .chain(once(Row::new([
        TableCell::new_with_alignment("Repository", 1, Alignment::Center),
        TableCell::new_with_alignment(reponame, 1, Alignment::Left),
    ])));

    let table = CodTableBuilder::new()
        .with_max_column_width(100)
        .add_rows(rows)
        .build();

    println!("{}", table.render());
}
