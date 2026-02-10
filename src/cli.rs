use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
pub enum Stack {
    Auto,
    Rust,
    Node,
    Python,
    Go,
    Multi,
    Other,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum AuthMode {
    Auto,
    Gh,
    Pat,
    App,
}

#[derive(Parser, Debug)]
#[command(
    name = "zip2repo",
    version,
    about = "Turn a source .zip into a GitHub repo wired to GitHub-native scaffolds."
)]
pub struct Cli {
    /// Local path or HTTPS URL to zip
    pub zip: String,

    #[arg(long)]
    pub owner: String,

    #[arg(long)]
    pub repo: String,

    #[arg(long, conflicts_with = "public")]
    pub private: bool,

    #[arg(long, conflicts_with = "private")]
    pub public: bool,

    #[arg(long, default_value_t = false)]
    pub apply_settings: bool,

    #[arg(long, default_value_t = false)]
    pub dry_run: bool,

    #[arg(long, value_enum, default_value_t = Stack::Auto)]
    pub stack: Stack,

    #[arg(long, value_enum, default_value_t = AuthMode::Auto)]
    pub auth: AuthMode,

    #[arg(long, default_value_t = false)]
    pub verbose: bool,
}
