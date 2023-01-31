use clap::Parser;

/// Create a label
#[derive(Parser, Debug)]
pub struct CreateLabelArgs {
    /// Label name
    #[arg(short, long, required = true)]
    pub name: String,
}
