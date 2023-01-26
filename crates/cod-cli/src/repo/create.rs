use clap::Parser;
use cod_types::api::privacy_type::Privacy;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    before_help = None,
    after_help = None,
    long_about = None,
    before_long_help = None,
    after_long_help = None
)]
pub struct RepoCreateArgs {
    #[arg(long)]
    pub default_branch: Option<String>,

    #[arg(short, long)]
    pub description: Option<String>,

    #[arg(short, long)]
    pub name: Option<String>,

    #[arg(short, long)]
    pub private: Option<Privacy>,
}
