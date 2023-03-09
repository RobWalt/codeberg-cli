pub mod list;
pub mod view;

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum NotificationArgs {
    List(list::ListNotificationArgs),
    View(view::ViewNotificationArgs),
}
