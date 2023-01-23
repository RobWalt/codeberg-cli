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
pub struct CreateIssueArgs {
    #[arg(short, long)]
    pub title: Option<String>,

    #[arg(short, long)]
    pub labels: Option<Vec<usize>>,

    #[arg(short, long)]
    pub body: Option<String>,

    #[arg(short, long)]
    pub assignees: Option<Vec<String>>,
}
