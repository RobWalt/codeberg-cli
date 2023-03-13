pub mod auth;
pub mod issue;
pub mod label;
pub mod milestone;
pub mod notification;
pub mod pull_request;
pub mod repo;
pub mod user;

use clap::{CommandFactory, Parser};
use clap_complete::Shell;

pub const LOGO: &str = include_str!("logo.txt");

pub fn generate_completion(shell: Shell, bin_name: &str) {
    let cmd = &mut MainArgs::command();
    clap_complete::generate(shell, cmd, bin_name, &mut std::io::stdout());
}

/// Codeberg CLI app
#[derive(Parser, Debug)]
#[command(name = "berg", version, before_long_help = LOGO)]
pub enum MainArgs {
    #[command(subcommand)]
    Auth(auth::AuthArgs),

    #[command(subcommand)]
    User(user::UserArgs),

    #[command(subcommand)]
    Issue(issue::IssueArgs),

    #[command(subcommand)]
    Pull(pull_request::PullRequestArgs),

    #[command(subcommand)]
    Label(label::LabelArgs),

    #[command(subcommand)]
    Repo(repo::RepoArgs),

    #[command(subcommand)]
    Milestone(milestone::MilestoneArgs),

    #[command(subcommand)]
    Notification(notification::NotificationArgs),

    /// Print completion script
    Completion {
        /// Shell to generate completion for
        shell: Shell,
    },
}
