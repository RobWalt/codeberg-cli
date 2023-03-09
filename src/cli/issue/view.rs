use crate::types::api::state_type::StateType;
use clap::Parser;

/// View details of selected issue
#[derive(Parser, Debug)]
pub struct ViewIssueArgs {
    /// Select from issues with the chosen state
    #[arg(short, long, value_enum, default_value_t = StateType::All)]
    pub state: StateType,

    /// Disabled: display issue summary | Enabled: display issue commen history
    #[arg(short, long)]
    pub comments: bool,
}
