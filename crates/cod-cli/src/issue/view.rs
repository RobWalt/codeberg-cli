use clap::Parser;
use cod_types::api::state_type::StateType;

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
    #[arg(short, long, value_enum, default_value_t=StateType::All)]
    pub state: StateType,

    #[arg(short, long)]
    pub comments: bool,
}
