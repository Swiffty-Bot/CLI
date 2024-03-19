mod account;
mod init;
mod logo;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "customs",
    version = "0.1.0",
    about = "Build discord bots with ease"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Initialize a project")]
    Init(init::Cli),
    #[command(about = "Complete account actions")]
    Account(account::Cli),
}

pub fn run() {
    let args = Cli::parse();
    match args.command {
        Commands::Init(args) => {
            logo::logo();
            init::init(args)
        }
        Commands::Account(_args) => {
            logo::logo();
            account::account()
        }
    }
}
