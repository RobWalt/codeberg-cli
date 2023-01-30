use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "List all issues of the current repository")]
pub struct ListIssueArgs {
    #[arg(
        short,
        long,
        default_value_t = 5,
        help = "The amount of issues that is displayed"
    )]
    pub count: usize,
}
