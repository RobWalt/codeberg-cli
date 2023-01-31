pub mod assignees;
pub mod clone;
pub mod create;
pub mod fork;
pub mod info;

use clap::Subcommand;

/// Repository subcommands
#[derive(Subcommand, Debug)]
pub enum RepoArgs {
    Create(create::RepoCreateArgs),
    Clone(clone::RepoCloneArgs),
    Fork(fork::RepoForkArgs),
    Info(info::RepoInfoArgs),
    Assignees(assignees::RepoAssigneesArgs),
}
