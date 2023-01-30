use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Create an issue")]
pub struct CreateIssueArgs {
    #[arg(short, long, help = "Title or summary")]
    pub title: Option<String>,

    // TODO: ??? why ids here, change to names
    #[arg(short, long, help = "List of label ids")]
    pub labels: Option<Vec<usize>>,

    #[arg(id = "description", short, long, help = "Main description of issue")]
    pub body: Option<String>,

    #[arg(short, long, help = "List of the names of assignees")]
    pub assignees: Option<Vec<String>>,
}
