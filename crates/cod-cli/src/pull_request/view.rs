use clap::Parser;
use cod_types::api::state_type::StateType;

#[derive(Parser, Debug)]
#[command(about = "View details of a selected pull request")]
pub struct ViewPullRequestsArgs {
    /// Select from pull requests with the chosen state
    #[arg(
        short,
        long,
        value_enum,
        default_value_t = StateType::All,
    )]
    pub state: StateType,

    /// Disabled: display issue summary | Enabled: display issue comment history
    #[arg(short, long)]
    pub comments: bool,
}
