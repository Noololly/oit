use clap::{Parser, Subcommand};
use std::path::PathBuf;
#[path = "commands/init.rs"] mod init;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init {
        path: Option<PathBuf>,
    }
}
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { path } => {
            init::init(path);
        }
    }
}
