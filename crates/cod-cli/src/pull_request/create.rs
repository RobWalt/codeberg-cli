use clap::Parser;

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
pub struct CreatePullRequestArgs {
    #[arg(long)]
    pub assignees: Option<Vec<String>>,

    #[arg(long)]
    pub base: Option<String>,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long)]
    pub head: Option<String>,

    #[arg(long)]
    pub labels: Option<Vec<String>>,

    #[arg(long)]
    pub title: Option<String>,
}
