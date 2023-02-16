use clap::Parser;
use cod_types::api::notification::notification_state_type::NotificationStateType;
use cod_types::api::notification::notification_type::NotificationSubjectType;

#[derive(Debug, Parser)]
pub struct ListNotificationArgs {
    #[arg(short, long, default_value_t = false)]
    pub all: bool,

    #[arg(long, default_values_t = vec![NotificationStateType::Unread, NotificationStateType::Pinned])]
    pub status_types: Vec<NotificationStateType>,

    #[arg(long)]
    pub subject_type: Option<NotificationSubjectType>,

    #[arg(short, long)]
    pub dates: bool,

    #[arg(short, long, default_value_t = 1)]
    pub page: usize,

    #[arg(short, long, default_value_t = usize::MAX)]
    pub limit: usize,
}
