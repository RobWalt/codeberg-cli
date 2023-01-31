use clap::Parser;
use cod_types::api::state_type::StateType;

/// List pull requests
#[derive(Parser, Debug)]
pub struct ListPullRequestArgs {
    /// Number of pull requests to be displayed
    #[arg(short, long, value_name = "N", default_value_t = 5)]
    pub count: usize,

    /// Filter pull requests with the chosen state
    #[arg(short, long, default_value_t = StateType::All)]
    pub state: StateType,
}
