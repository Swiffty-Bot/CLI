use clap::{command, Arg, Command};
mod init;
fn main() {
        let matches = command!()
            .subcommand(Command::new("init")
                .about("Initialize a project")
                .alias("i")
                .arg(Arg::new("project-name").long("name").short('n').alias("nme"))
                .arg(Arg::new("project-lang").long("language").short('l').alias("lang"))
            )
            .get_matches();


            if let Some(matches) = matches.subcommand_matches("init") {
                init::init(matches);

            }
    }