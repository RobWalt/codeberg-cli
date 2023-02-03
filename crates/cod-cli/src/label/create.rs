use clap::Parser;

/// Create a label
#[derive(Parser, Debug)]
pub struct CreateLabelArgs {
    /// Label name
    #[arg(short, long)]
    pub name: Option<String>,

    /// Label color (in hex format "#abcdef")
    #[arg(short, long)]
    pub color: Option<String>,

    /// Label purpose description
    #[arg(short, long)]
    pub description: Option<String>,
}
