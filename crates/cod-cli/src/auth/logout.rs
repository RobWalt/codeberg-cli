use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Logout. This deletes the current authentication TOKEN")]
pub struct LogoutArgs {}
