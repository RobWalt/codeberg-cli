use clap::Parser;
use cod_types::api::notification::notification_state_type::NotificationStateType;

#[derive(Debug, Parser)]
pub struct ListNotificationArgs {
    #[arg(short, long, default_value_t = false)]
    pub all: bool,

    #[arg(short, long, default_values_t = vec![NotificationStateType::Unread, NotificationStateType::Pinned])]
    pub status_types: Vec<NotificationStateType>,

    pub since: Option<String>,

    pub before: Option<String>,

    #[arg(short, long, default_value_t = 1)]
    pub page: usize,

    #[arg(short, long, default_value_t = usize::MAX)]
    pub limit: usize,
}
