use clap::Parser;
use crossterm::style::Stylize;
use git2::Repository;
use std::env;
use std::fs;
use std::process;
use toml::Value;

#[derive(Parser)]
#[command(name = "add", about = "Add something")]
pub struct Cli {
    #[command()]
    pub package: String,
}

#[tokio::main]
pub async fn add(args: Cli) {
    if !fs::metadata("Customs.toml").is_ok() {
        println!("{} Customs.toml does not exist", "ERROR".bold().red());
        process::exit(0);
    }
    let toml_content = fs::read_to_string("Customs.toml").expect("Failed to read file");

    let toml_value: Value = toml::from_str(&toml_content).expect("Failed to parse TOML");

    let project_language = match toml_value.get("project") {
        Some(project) => match project.get("language") {
            Some(language) => language.as_str().unwrap_or(""),
            None => {
                println!(
                    "{} Language not found in [projects] category",
                    "ERROR".bold().red()
                );
                process::exit(0);
            }
        },
        None => {
            println!(
                "{} [projects] category not found in Customs.toml file",
                "ERROR".bold().red()
            );
            process::exit(0);
        }
    };

    let url = "https://customs-server.vercel.app/api/modules/find/".to_string()
        + project_language
        + "/"
        + &args.package;

    let client = reqwest::Client::new();
    let req = client.get(url);

    match req.send().await {
        Ok(res) => {
            if res.status() != reqwest::StatusCode::OK {
                match res.json::<Value>().await {
                    Ok(json) => {
                        if let Some(error_message) = json.get("error") {
                            println!("{} {}", "ERROR".red().bold(), error_message);
                            process::exit(0);
                        } else {
                            println!("{} {}", "ERROR".red().bold(), "Internal error");
                            process::exit(0);
                        }
                    }
                    Err(e) => {
                        println!("{} {}", "ERROR".red().bold(), e);
                    }
                }
            } else {
                match res.json::<Value>().await {
                    Ok(json) => {
                        if let Some(gitrepo) = json.get("gitrepo") {
                            let gitURL = "https://github.com/".to_string()
                                + &gitrepo.to_string().trim_matches('"');

                            let current_dir =
                                env::current_dir().expect("Failed to get current directory");
                            let current_dir_str = current_dir.to_string_lossy();
                            let custom_modules_dir =
                                format!("{}/CustomsModules/{}", current_dir_str, &args.package);

                            let _repo = match Repository::clone(&gitURL, &custom_modules_dir) {
                                Ok(repo) => repo,
                                Err(e) => {
                                    println!(
                                        "{}",
                                        format!("{} Failed to clone: {}", "ERROR".bold().red(), e)
                                    );
                                    process::exit(1);
                                }
                            };
                        }
                    }
                    Err(e) => {
                        println!(
                            "{} Error parsing response JSON: {}",
                            "ERROR".red().bold(),
                            e
                        );
                    }
                }
            }
        }
        Err(e) => {
            println!("{} Failed to send request: {}", "ERROR".red().bold(), e);
        }
    }
}
