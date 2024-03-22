mod auth;
mod init;
mod logo;
mod add;

use clap::Parser;

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

#[derive(Parser)]
pub enum Commands {
    #[command(about = "Initialize a project")]
    Init(init::Cli),
    #[command(about = "Complete account actions", subcommand)]
    Auth(AuthCommands),
    #[command(about = "Initialize a project")]
    Add(add::Cli),

}

#[derive(Parser)]
pub enum AuthCommands {
    #[command(about = "Create a new authentication")]
    Create,
    #[command(about = "Delete an authentication")]
    Delete,
    #[command(about = "Log in")]
    Login,
    #[command(about = "Log out")]
    Logout,
}

pub fn run() {
    let args = Cli::parse();
    match args.command {
        Commands::Init(args) => {
            logo::logo();
            init::init(args)
        }
        Commands::Auth(auth_cmd) => match auth_cmd {
            AuthCommands::Create => auth::create::create(),
            AuthCommands::Delete => auth::delete::delete(),
            AuthCommands::Login => auth::login::login(),
            AuthCommands::Logout => auth::logout::logout(),
        },
        Commands::Add(args) => {
            logo::logo();
            add::add(args)
        }
    }
}
