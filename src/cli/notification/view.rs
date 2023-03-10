use clap::Parser;
#[derive(Debug, Parser)]
pub struct ViewNotificationArgs {
    pub id: Option<usize>,

    #[arg(short, long)]
    pub all: bool,
}
