pub mod assignees;
pub mod create;
pub mod info;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
#[command(author, version, about = "Repo commands", long_about = None)]
pub enum RepoArgs {
    Create(create::RepoCreateArgs),
    Info(info::RepoInfoArgs),
    Assignees(assignees::RepoAssigneesArgs),
}
