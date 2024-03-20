use clap::Args;
use crossterm::style::Stylize;
use dialoguer;
use directories::ProjectDirs;
use reqwest;
use serde_json::Value;
use std::fs::{self, OpenOptions};
use std::path::Path;
use std::process;
use std::{fs::File, io::Read};

#[derive(Args)]
pub struct Cli {}

pub fn account() {
    let theme = dialoguer::theme::ColorfulTheme::default();

    let actions = vec!["create", "delete", "login", "logout", "view"];
    let action_selection = dialoguer::Select::with_theme(&theme)
        .with_prompt("What would you like to do")
        .items(&actions)
        .interact()
        .unwrap();

    match actions[action_selection] {
        "create" => create(),
        "delete" => delete(),
        "login" => login(),
        "logout" => logout(),
        "view" => view(),
        _ => println!("Invalid option selected"),
    }
}

#[tokio::main]
async fn create() {
    let project_dirs =
        ProjectDirs::from("", "", "Customs").expect("Failed to get project directories");
    let data_dir_path = project_dirs.data_dir();

    if !data_dir_path.exists() {
        fs::create_dir_all(&data_dir_path).expect("Failed to create data directory");
    }

    let file_path = data_dir_path.join("accounts_info.json");

    if Path::new(&file_path).exists() {
        println!(
            "{} {}",
            "ERROR".red().bold(),
            "You are logged into an account"
        );
        process::exit(0);
    }

    let theme = dialoguer::theme::ColorfulTheme::default();
    let username: String = dialoguer::Input::with_theme(&theme)
        .with_prompt("Username")
        .interact_text()
        .unwrap();

    let password: String = dialoguer::Password::with_theme(&theme)
        .with_prompt("Password")
        .with_confirmation("Confirm Password", "Passwords do not match")
        .interact()
        .unwrap();

    let client = reqwest::Client::new();
    let url = "https://customs-server.vercel.app/api/accounts/create/".to_string()
        + &username.to_string()
        + "/"
        + &password.to_string();
    println!("{}", url);
    let req = client.post(url);

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
                        let file = OpenOptions::new()
                            .create(true)
                            .write(true)
                            .append(true)
                            .open(&file_path)
                            .expect("Failed to open file");

                        serde_json::to_writer(file, &json).expect("Failed to write to file");
                        println!("{} {}", "Created account".bold().green(), &username.green());
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
#[tokio::main]
async fn delete() {}

#[tokio::main]
async fn login() {
    let project_dirs =
        ProjectDirs::from("", "", "Customs").expect("Failed to get project directories");
    let data_dir_path = project_dirs.data_dir();

    if !data_dir_path.exists() {
        fs::create_dir_all(&data_dir_path).expect("Failed to create data directory");
    }

    let file_path = data_dir_path.join("accounts_info.json");

    if Path::new(&file_path).exists() {
        println!("{} {}", "ERROR".red().bold(), "You are already logged in");
        process::exit(0);
    }

    let theme = dialoguer::theme::ColorfulTheme::default();

    let username: String = dialoguer::Input::with_theme(&theme)
        .with_prompt("Username")
        .interact_text()
        .unwrap();

    let password: String = dialoguer::Password::with_theme(&theme)
        .with_prompt("Password")
        .interact()
        .unwrap();

    let client = reqwest::Client::new();
    let url = "https://customs-server.vercel.app/api/accounts/login/".to_string()
        + &username.to_string()
        + "/"
        + &password.to_string();

    let req = client.post(url);

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
                        let file = OpenOptions::new()
                            .create(true)
                            .write(true)
                            .append(true)
                            .open(&file_path)
                            .expect("Failed to open file");

                        serde_json::to_writer(file, &json).expect("Failed to write to file");
                        println!("{} {}", "Logged in as".bold().green(), &username.green())
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

#[tokio::main]
async fn logout() {
    let theme = dialoguer::theme::ColorfulTheme::default();
    let project_dirs =
        ProjectDirs::from("", "", "Customs").expect("Failed to get project directories");
    let data_dir_path = project_dirs.data_dir();

    let file_path = data_dir_path.join("accounts_info.json");

    if !file_path.exists() {
        println!("{} You are not logged in", "ERROR".bold().red());
        process::exit(0);
    }

    let input: bool = dialoguer::Input::with_theme(&theme)
        .with_prompt("Are you sure you want to proceed")
        .interact_text()
        .unwrap();

    if input {
        match fs::remove_file(file_path) {
            Ok(()) => println!("{}", "Logged out".bold().green()),
            Err(err) => println!("{} Could not delete file:\n{}", "ERROR".bold().red(), err),
        }
    } else {
        process::exit(0);
    }
}

#[tokio::main]
async fn view() {
    let theme = dialoguer::theme::ColorfulTheme::default();
    let project_dirs =
        ProjectDirs::from("", "", "Customs").expect("Failed to get project directories");
    let data_dir_path = project_dirs.data_dir();

    let file_path = data_dir_path.join("accounts_info.json");

    if !file_path.exists() {
        println!("{} You are not logged in", "ERROR".bold().red());
        process::exit(0);
    }



}