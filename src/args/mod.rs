use std::fmt::Display;

use structopt::StructOpt;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

#[derive(StructOpt)]
#[structopt(about = "Codeberg CLI Tool")]
pub enum Opts {
    Repo {
        #[structopt(parse(try_from_str = abc))]
        repo: RepoOpts,
    },
}

#[derive(StructOpt, EnumString, EnumIter, Display)]
#[strum(ascii_case_insensitive)]
pub enum RepoOpts {
    Create,
    Clone,
    Fork,
}

impl RepoOpts {
    fn name() -> String {
        String::from("repo")
    }
}

fn abc(i: &str) -> anyhow::Result<RepoOpts> {
    RepoOpts::try_from(i)
        .map_err(|_| anyhow::anyhow!(subcommand_help::<RepoOpts>(RepoOpts::name())))
}

fn subcommand_help<SST: IntoEnumIterator + Display>(subcommand_name: String) -> String {
    format!(
        "\n\nFor subcommand \"{subcommand_name}\", please choose one of the following actions instead:\n\n{}",
        SST::iter()
            .map(|opt| format!(" - {}", opt))
            .reduce(|a, b| a + "\n" + &b)
            .unwrap_or_default()
    )
}
