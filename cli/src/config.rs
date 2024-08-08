use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "cli", author = "Sunyb", version = "0.1.0", about = "test rust cli parse", long_about = None)]
#[command(next_line_help = true)]
#[command(propagate_version = true)]
pub struct Cli {
    // 必选默认参数
    #[arg(short, long, value_name = "env", help = "env", default_value = "dev")]
    pub env: String,
    // 子参数
    #[command(subcommand)]
    pub command: Commands,
    // 可选参数
    #[arg(short, long, value_name = "verbose", help = "verbose")]
    pub verbose: Option<bool>,
    // 枚举参数
    #[arg(short, long, value_enum)]
    pub mode: Mode,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Adds files to myapp
    Add { name: Option<String> },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    /// Run swiftly
    Fast,
    /// Crawl slowly but steadily
    ///
    /// This paragraph is ignored because there is no long help text for possible values.
    Slow,
}
