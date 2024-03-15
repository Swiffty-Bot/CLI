mod init;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "customs",
    version = "0.1.0",
    about = "thing",
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Initialize a project")]
    Init(init::Cli),
}

pub fn run() {
    let args = Cli::parse();
    match args.command {
        Commands::Init(args) => init::init(args),
    }
}