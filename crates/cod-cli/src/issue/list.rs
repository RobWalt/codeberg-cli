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
pub struct ListIssueArgs {
    #[arg(short, long, default_value_t = 5)]
    pub count: usize,
}
