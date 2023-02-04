pub mod auth;
pub mod issue;
pub mod label;
pub mod logo;
pub mod milestone;
pub mod pull_request;
pub mod repo;
pub mod user;

use clap::{CommandFactory, Parser};
use clap_complete::Shell;

pub fn generate_completion(shell: Shell, bin_name: &str) {
    let cmd = &mut MainArgs::command();
    clap_complete::generate(shell, cmd, bin_name, &mut std::io::stdout());
}

/// Codeberg CLI app
#[derive(Parser, Debug)]
#[command(name = "cod", version, before_long_help = logo::LOGO)]
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

    /// Print completion script
    Completion {
        /// Shell to generate completion for
        shell: Shell,
    },
}
