use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Create a pull request")]
pub struct CreatePullRequestArgs {
    #[arg(short, long, help = "List of the names of assignees")]
    pub assignees: Option<Vec<String>>,

    #[arg(short, long, help = "The target branch of the pull request")]
    pub base: Option<String>,

    #[arg(id = "description", short, long, help = "Main description of issue")]
    pub body: Option<String>,

    #[arg(short, long, help = "The source branch of the pull request")]
    pub head: Option<String>,

    #[arg(short, long, help = "List of labels")]
    pub labels: Option<Vec<String>>,

    #[arg(short, long, help = "Title or summary")]
    pub title: Option<String>,
}
