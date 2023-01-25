use clap::Parser;
use cod_types::api::issue_status::IssueStatus;

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
pub struct ViewIssueArgs {
    #[arg(short, long, value_enum, default_value_t=IssueStatus::All)]
    pub state: IssueStatus,
}
