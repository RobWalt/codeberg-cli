use clap::Parser;

/// List all labels in the current repository
#[derive(Parser, Debug)]
pub struct ListLabelsArgs {
    /// Number of labels to be displayed
    #[arg(short, long, value_name = "N", default_value_t = 5)]
    pub count: usize,
}
