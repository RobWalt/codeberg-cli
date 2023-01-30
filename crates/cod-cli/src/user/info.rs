use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Display a short summary of the authenticated user account")]
pub struct UserInfoArgs {}
