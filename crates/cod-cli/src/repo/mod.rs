pub mod assignees;
pub mod info;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
#[command(author, version, about = "Repo commands", long_about = None)]
pub enum RepoArgs {
    Info(info::RepoInfoArgs),
    Assignees(assignees::RepoAssigneesArgs),
}
