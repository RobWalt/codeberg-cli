use clap::Parser;
use cod_types::api::privacy_type::Privacy;

/// Create a new repository
#[derive(Parser, Debug)]
pub struct RepoCreateArgs {
    /// Main branch to init repository with (usually "main")
    #[arg(id = "default-branch", long)]
    pub default_branch: Option<String>,

    /// Repository description
    #[arg(short, long)]
    pub description: Option<String>,

    /// Repository name
    #[arg(short, long)]
    pub name: Option<String>,

    /// Repository visibility
    #[arg(short, long, value_enum, value_name = "VISIBILITY")]
    pub private: Option<Privacy>,
}
