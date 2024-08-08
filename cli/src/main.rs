mod config;

use clap::Parser;
use config::{Cli, Commands, Mode};

fn main() {
    let cli = Cli::parse();
    // 提取env
    println!("env is: {:?}", cli.env);
    // 提取verbose
    println!("verbose is: {:?}", cli.verbose);
    // 提取commond
    #[warn(unreachable_patterns)]
    match cli.command {
        Commands::Add { name } => {
            println!("'myapp add' was used, name is: {name:?}")
        }
    }
    // 提取mod
    match cli.mode {
        Mode::Fast => println!("fast"),
        Mode::Slow => println!("slow"),
    }
}
