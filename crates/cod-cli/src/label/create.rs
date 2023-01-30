use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Create a label")]
pub struct CreateLabelArgs {
    #[arg(short, long, required = true, help = "Name of the label")]
    pub name: String,
}
