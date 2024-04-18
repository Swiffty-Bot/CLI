mod init;
mod build;
mod logo;


use clap::Parser;

#[derive(Parser)]
#[command(
    name = "customs",
    version = "0.1.0",
    about = "Build a package with swiffty"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser)]
pub enum Commands {
    #[command(about = "Initialize a plugin")]
    Init(init::Cli),
    #[command(about = "Build a plugin")]
    Build(build::Cli),
}


pub fn run() {
    let args = Cli::parse();
    match args.command {
        Commands::Init(args) => {
            logo::logo();
            init::init(args);
        }

            Commands::Build(args) => {
                logo::logo();
                build::build(args)
        }
        
    }
}
