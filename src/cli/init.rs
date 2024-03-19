use clap::Args;
use crossterm::style::Stylize;
use dialoguer::Input;
use git2::Repository;
use std::path::PathBuf;
use std::{fs, process};

#[derive(Args)]
pub struct Cli {}

pub fn init(_args: Cli) {
    let theme = dialoguer::theme::ColorfulTheme::default();

    let prog_name: String = Input::with_theme(&theme)
        .with_prompt("Project name")
        .interact_text()
        .unwrap();

    let allowed_languages = vec!["js", "py", "rs"];
    let lang_selection = dialoguer::Select::with_theme(&theme)
        .with_prompt("Language")
        .items(&allowed_languages)
        .interact()
        .unwrap();

    let path = PathBuf::from(&prog_name);

    if let Ok(entries) = fs::read_dir(&path) {
        if entries.peekable().peek().is_some() {
            println!("{}", "Directory is not empty".red().bold());

            let input: bool = Input::with_theme(&theme)
                .with_prompt("Proceed anyway")
                .interact_text()
                .unwrap();

            if !input {
                process::exit(0);
            }
        }
    }

    let git_url = "https://github.com/C-h-a-r/DiscordCustoms-Template";

    let url_with_lang = format!("{}-{}/", git_url, allowed_languages[lang_selection]);

    let _repo = match Repository::clone(&url_with_lang, &path) {
        Ok(repo) => repo,
        Err(e) => {
            println!("{}", format!("Failed to clone: {}", e).red());
            process::exit(1);
        }
    };

    println!("{} {}", "Initialized".green(), prog_name);
}
