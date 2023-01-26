pub mod assignees;
pub mod clone;
pub mod create;
pub mod info;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
#[command(author, version, about = "Repo commands", long_about = None)]
pub enum RepoArgs {
    Create(create::RepoCreateArgs),
    Clone(clone::RepoCloneArgs),
    Info(info::RepoInfoArgs),
    Assignees(assignees::RepoAssigneesArgs),
}
