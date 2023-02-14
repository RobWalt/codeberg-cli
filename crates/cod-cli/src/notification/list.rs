use clap::Parser;
use cod_types::api::notification::notification_state_type::NotificationStateType;

#[derive(Debug, Parser)]
pub struct ListNotificationArgs {
    #[arg(short, long, default_value_t = false)]
    all: bool,

    #[arg(short, long, default_values_t = vec![NotificationStateType::Unread, NotificationStateType::Pinned])]
    status_types: Vec<NotificationStateType>,

    since: Option<String>,

    before: Option<String>,

    #[arg(short, long, default_value_t = 1)]
    page: usize,

    #[arg(short, long, default_value_t = usize::MAX)]
    limit: usize,
}
