use clap::Parser;
use cod_types::api::state_type::StateType;

/// View details of selected milestone
#[derive(Parser, Debug)]
pub struct ViewMilestonesArgs {
    /// Select from milestones with the chosen state
    #[arg(short, long, value_enum, default_value_t = StateType::All)]
    pub state: StateType,
}
