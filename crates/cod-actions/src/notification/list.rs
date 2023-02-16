use chrono::DateTime;
use chrono::Utc;
use cod_cli::notification::list::ListNotificationArgs;
use cod_client::CodebergClient;
use cod_render::datetime::ask_datetime;
use cod_render::ui::multi_fuzzy_select_with_key;
use cod_types::api::notification::notification_state_type::NotificationStateType;
use cod_types::api::notification::notification_thread::NotificationThread;
use cod_types::api::notification::notification_type::NotificationSubjectType;
use strum::Display;

#[derive(Debug, Clone, Copy, Display, PartialEq, Eq)]
enum DateFilter {
    Since,
    Before,
}

#[derive(Debug, Clone)]
struct FilterArgs {
    all: bool,
    since: Option<DateTime<Utc>>,
    before: Option<DateTime<Utc>>,
    status_types: Vec<NotificationStateType>,
    subject_type: Option<NotificationSubjectType>,
    page: usize,
    limit: usize,
}

impl FilterArgs {
    pub fn new_from_args(args: ListNotificationArgs) -> Self {
        Self {
            all: false,
            since: None,
            before: None,
            status_types: args.status_types,
            subject_type: args.subject_type,
            page: args.page,
            limit: args.limit,
        }
    }
}

pub async fn list_notifications(
    args: ListNotificationArgs,
    client: &CodebergClient,
) -> anyhow::Result<()> {
    let filter_args = collect_extra_args(args)?;

    let notification_threads_list = client
        .get_all_notifications_filtered(
            filter_args.all,
            filter_args.since,
            filter_args.before,
            filter_args.status_types,
            filter_args.subject_type,
            filter_args.page,
            filter_args.limit,
        )
        .await?;

    tracing::debug!("{notification_threads_list:?}");

    present_notification_threads(notification_threads_list);

    Ok(())
}

fn collect_extra_args(args: ListNotificationArgs) -> anyhow::Result<FilterArgs> {
    let filter_dates = args.dates;
    let mut filter_args = FilterArgs::new_from_args(args);

    if filter_dates {
        let selected_filter_args = multi_fuzzy_select_with_key(
            vec![DateFilter::Before, DateFilter::Since],
            "date filter",
            |_| false,
        )?;

        if selected_filter_args.contains(&DateFilter::Since) {
            let since_date = ask_datetime("Minimum Date")?;
            filter_args.since.replace(since_date);
        }

        if selected_filter_args.contains(&DateFilter::Before) {
            let before_date = ask_datetime("Maximum Date")?;
            filter_args.before.replace(before_date);
        }
    }

    Ok(filter_args)
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
        format!("   {header}   \n{}", "=".repeat(header.len() + 2 * 3),),
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
