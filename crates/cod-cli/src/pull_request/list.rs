use clap::Parser;
use cod_types::api::state_type::StateType;

#[derive(Parser, Debug)]
#[command(about = "List all pull requests of the current repository")]
pub struct ListPullRequestArgs {
    #[arg(
        short,
        long,
        default_value_t = 5,
        help = "The amount of pull requests that is displayed"
    )]
    pub count: usize,

    #[arg(
        short,
        long,
        default_value_t = StateType::All,
        help = "Only list pull request with chosen state"
    )]
    pub state: StateType,
}
