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
pub struct ListPullRequestArgs {
    #[arg(short, long, default_value_t = 5)]
    pub count: usize,

    #[arg(short, long, default_value_t = StateType::All)]
    pub state: StateType,
}
