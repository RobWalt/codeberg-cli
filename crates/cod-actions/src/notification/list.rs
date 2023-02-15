use cod_cli::notification::list::ListNotificationArgs;
use cod_client::CodebergClient;
use cod_types::api::notification::notification_thread::NotificationThread;

pub async fn list_notifications(
    _args: ListNotificationArgs,
    client: &CodebergClient,
) -> anyhow::Result<()> {
    let notification_threads_list = client.get_all_notifications().await?;

    tracing::debug!("{notification_threads_list:?}");

    present_notification_threads(notification_threads_list);

    Ok(())
}

fn present_notification_threads(notification_threads_list: Vec<NotificationThread>) {
    use cod_render::prelude::*;
    use std::iter::once;

    let header = if notification_threads_list.is_empty() {
        "Notification Threads (empty)"
    } else {
        "Notification Threads"
    };

    let rows = once(Row::new([TableCell::new_with_alignment(
        header,
        1,
        Alignment::Center,
    )]))
    .chain(
        notification_threads_list
            .into_iter()
            .map(|notification_thread| {
                Row::new([TableCell::new_with_alignment(
                    notification_thread.subject.title,
                    1,
                    Alignment::Center,
                )])
            }),
    );

    let table = CodTableBuilder::new().add_rows(rows).build();

    println!("{}", table.render());
}
