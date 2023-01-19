pub mod auth;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub enum MainArgs {
    #[command(subcommand)]
    Auth(auth::AuthArgs),
}
