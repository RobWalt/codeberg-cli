use clap::Parser;
use cod_types::api::privacy_type::Privacy;

#[derive(Parser, Debug)]
#[command(about = "Create a new repository for the authenticated user")]
pub struct RepoCreateArgs {
    #[arg(
        id = "default-branch",
        long,
        help = "Main branch the repository is initialized with (usually main)"
    )]
    pub default_branch: Option<String>,

    #[arg(short, long, help = "Repository description")]
    pub description: Option<String>,

    #[arg(short, long, help = "Repository name")]
    pub name: Option<String>,

    #[arg(short, long, help = "Repository visibility")]
    pub private: Option<Privacy>,
}
