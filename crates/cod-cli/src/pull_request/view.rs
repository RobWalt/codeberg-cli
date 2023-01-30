use clap::Parser;
use cod_types::api::state_type::StateType;

#[derive(Parser, Debug)]
#[command(about = "View details of a selected pull request")]
pub struct ViewPullRequestsArgs {
    #[arg(
        short,
        long,
        value_enum,
        default_value_t = StateType::All,
        help = "Select from pull requests with the chosen state"
    )]
    pub state: StateType,

    #[arg(
        short,
        long,
        help = "Disabled: Display issue summary | Enabled: Display issue comment history"
    )]
    pub comments: bool,
}
