use std::{fs, process};
use std::path::PathBuf;
use git2::Repository;
use clap::Args;
use dialoguer::Input;

use crate::model::Config;
use crossterm::style::Stylize;

#[derive(Args)]
pub struct Cli {
    #[clap(short, long)]
    pub name: String,

    #[clap(short, long)]
    pub lang: String,

    #[clap(short, long)]
    pub path: Option<PathBuf>,
}

pub fn init(args: Cli) {
    let config_content = fs::read_to_string("Config.toml")
        .expect("Failed to read config.toml file");

    let config: Config = toml::from_str(&config_content)
        .expect("Failed to parse config.toml file");

    if !config.allowed_languages.contains(&args.lang) {
        println!("{}", "Language not allowed".red());
        process::exit(1);
    }

    let path = args.path
        .unwrap_or_else(|| PathBuf::from(&args.name));

    if let Ok(entries) = fs::read_dir(&path) {
        if entries.peekable().peek().is_some() { // faster than checking count > 0 bc it only checks one entry in the case that some retard monkey does this in a folder with a million files 
            println!("{}", "Directory is not empty".red());
            
            let theme = dialoguer::theme::ColorfulTheme::default();

            let input: bool = Input::with_theme(&theme)
                .with_prompt("Proceed anyway")
                .interact_text()
                .unwrap();

            if !input {
                process::exit(0); // idk the opcode for abort so this for now
            }
        }
    }

    let _repo = match Repository::clone(&config.git_url, &path) {
        Ok(repo) => repo,
        Err(e) => {
            println!("{}", format!("Failed to clone: {}", e).red());
            process::exit(1);
        }
    };

    println!("{}", "Project initialized successfully".green());
}
