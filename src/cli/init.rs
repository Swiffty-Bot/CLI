use clap::Args;
use dialoguer::Input;
use git2::Repository;
use std::path::PathBuf;
use std::{fs, process};
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
    let allowed_languages = ["js", "py", "rs"];
    let git_url = "https://github.com/C-h-a-r/DiscordCustoms-Template";

    if !allowed_languages.contains(&args.lang.as_str()) {
        println!("{}", "Language not supported".red());
        process::exit(1);
    }

    let path = args.path.unwrap_or_else(|| PathBuf::from(&args.name));

    

    if let Ok(entries) = fs::read_dir(&path) {
        if entries.peekable().peek().is_some() {
            println!("{}", "Directory is not empty".red());

            let theme = dialoguer::theme::ColorfulTheme::default();

            let input: bool = Input::with_theme(&theme)
                .with_prompt("Proceed anyway")
                .interact_text()
                .unwrap();

            if !input {
                process::exit(0);
            }
        }
    }

    let url_with_lang = format!("{}-{}/", git_url, args.lang);

    let _repo = match Repository::clone(&url_with_lang, &path) {
        Ok(repo) => repo,
        Err(e) => {
            println!("{}", format!("Failed to clone: {}", e).red());
            process::exit(1);
        }
    };

    println!("{} {}", "Initialized".green(), args.name);
}


