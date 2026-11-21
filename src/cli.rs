use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "neat",
    version,
    about = "A clean and user-friendly Conventional Commits assistant"
)]
pub struct Cli {
    /// Commit type (feat, fix, docs, ...)
    #[arg(short = 't', long = "type")]
    pub commit_type: Option<String>,

    /// Optional scope of the change
    #[arg(short, long)]
    pub scope: Option<String>,

    /// Short commit description
    #[arg(short, long)]
    pub message: Option<String>,

    /// Mark the commit as a breaking change
    #[arg(short, long)]
    pub breaking: bool,

    /// Description of the breaking change (implies --breaking)
    #[arg(long)]
    pub breaking_desc: Option<String>,
}

impl Cli {
    /// Interactive mode is used whenever the required values are not all provided.
    pub const fn is_interactive(&self) -> bool {
        self.commit_type.is_none() || self.message.is_none()
    }
}
