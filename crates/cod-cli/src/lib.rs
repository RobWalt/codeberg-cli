pub mod auth;
pub mod issue;
pub mod label;
pub mod pull_request;
pub mod repo;
pub mod user;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
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
}
