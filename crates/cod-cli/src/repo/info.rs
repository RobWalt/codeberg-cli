use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Display a short summary of the current repository")]
pub struct RepoInfoArgs {}
