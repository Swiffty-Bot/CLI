use inline_colorization::*;
use clap::ArgMatches;
use std::fs;
use toml::Value;
use git2::Repository;
use std::env;
use std::io::{self, Write};

pub fn init(matches: &ArgMatches) {
    let config_content = fs::read_to_string("Config.toml")
        .expect("Failed to read config.toml file");

    let config: Value = config_content
        .parse()
        .expect("Failed to parse config.toml file");

    let allowed_languages = match config.get("allowed_languages") {
        Some(Value::Array(arr)) => arr.iter()
            .filter_map(|v| v.as_str())
            .map(String::from)
            .collect::<Vec<String>>(),
        _ => panic!("allowed_languages array not found in config.toml"),
    };

    let git_url = match config.get("git_url") {
        Some(Value::String(url)) => url.clone(),
        _ => panic!("git_url not found in config.toml"),
    };

    let project_name = match matches.get_one::<String>("project-name") {
        Some(name) => {
            let name_str = name.to_string();
            fs::create_dir_all(&name_str)
                .expect("Failed to create project directory");
            name_str
        },
        None => {
            env::current_dir()
                .expect("Failed to get current directory")
                .display()
                .to_string()
        }
    };

    let project_lang = match matches.get_one::<String>("project-lang") {
        Some(lang) => lang.to_string(),
        None => "js".to_string(),
    };

    if !allowed_languages.contains(&project_lang.as_str().to_string()) {
        println!("{style_bold}{color_red}ERROR{color_reset}{style_reset} Language {style_bold}{project_lang}{style_reset} not supported");
        return;
    }

    if let Ok(entries) = fs::read_dir(&project_name) {
        if entries.count() > 0 {
            println!("{style_bold}{color_red}The current directory '{}' is not empty{color_reset}{style_reset}", project_name);
            println!("{style_bold}Do you want to continue and potentially overwrite existing files? (yes/no){style_reset}");

            let mut input = String::new();
            print!("> ");
            io::stdout().flush().expect("Failed to flush stdout");
            io::stdin().read_line(&mut input).expect("Failed to read line");

            let response = input.trim().to_lowercase();
            if response != "yes" {
                println!("{style_bold}{color_red}ERROR{color_reset}{style_reset} Operation aborted");
                return;
            }
        }
    }

    let url_with_lang = format!("{}-{}/", git_url, project_lang);

    let _repo = match Repository::clone(&url_with_lang, &project_name) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to clone: {}", e),
    };

    println!("{style_bold}{color_green}Initializing{color_reset}{style_reset} {project_name}\n\n{style_bold}New Run:{style_reset}\ncd {project_name}");
}
