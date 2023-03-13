use crate::cli::notification::view::ViewNotificationArgs;
use crate::client::BergClient;
use crate::render::datetime::render_datetime_and_info;
use crate::render::ui::fuzzy_select_with_key;
use crate::types::api::notification::notification_thread::NotificationThread;

pub async fn view_notifications(
    args: ViewNotificationArgs,
    client: &BergClient,
) -> anyhow::Result<()> {
    let selected_notification_thread = if let Some(thread_id) = args.id {
        let notification_thread = client.get_notification_thread(thread_id).await?;
        tracing::debug!("{notification_thread:?}");
        notification_thread
    } else {
        let notification_threads_list = client.get_all_notifications_unfiltered(args.all).await?;

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
    use crate::render::table::builder::BergTableBuilder;
    use std::iter::once;
    use term_table::row::Row;
    use term_table::table_cell::{Alignment, TableCell};

    let rows = once(Row::new([
        TableCell::new_with_alignment("Title", 1, Alignment::Center),
        TableCell::new_with_alignment(
            notification_thread.subject.title.as_str(),
            1,
            Alignment::Left,
        ),
    ]))
    .chain(once(Row::new([
        TableCell::new_with_alignment("URL", 1, Alignment::Center),
        TableCell::new_with_alignment(notification_thread.subject.html_url, 1, Alignment::Left),
    ])))
    .chain(once(Row::new([
        TableCell::new_with_alignment("Type", 1, Alignment::Center),
        TableCell::new_with_alignment(notification_thread.subject.notify_type, 1, Alignment::Left),
    ])))
    .chain(once(Row::new([
        TableCell::new_with_alignment("State", 1, Alignment::Center),
        TableCell::new_with_alignment(notification_thread.subject.state, 1, Alignment::Left),
    ])))
    .chain(once(Row::new([
        TableCell::new_with_alignment("Unread", 1, Alignment::Center),
        TableCell::new_with_alignment(
            if notification_thread.unread {
                "Yes"
            } else {
                "No"
            },
            1,
            Alignment::Left,
        ),
    ])))
    .chain(once(Row::new([
        TableCell::new_with_alignment("Pinned", 1, Alignment::Center),
        TableCell::new_with_alignment(
            if notification_thread.pinned {
                "Yes"
            } else {
                "No"
            },
            1,
            Alignment::Left,
        ),
    ])))
    .chain(once(Row::new([
        TableCell::new_with_alignment("Last updated", 1, Alignment::Center),
        TableCell::new_with_alignment(
            render_datetime_and_info(notification_thread.updated_at),
            1,
            Alignment::Left,
        ),
    ])));

    let table = BergTableBuilder::new()
        .with_max_column_width(100)
        .add_rows(rows)
        .build();

    println!("{}", table.render());
}
