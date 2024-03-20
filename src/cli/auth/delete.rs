use crossterm::style::Stylize;
use directories::ProjectDirs;
use std::fs::{self};
use std::process;
#[tokio::main]
pub async fn delete() {
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
    let url = "https://customs-server.vercel.app/api/accounts/delete/".to_string()
        + &username.to_string()
        + "/"
        + &password.to_string();

    let req = client.delete(url);

    match req.send().await {
        Ok(response) => {
            if response.status().is_success() {
                println!("{} {}", "Deleted account".bold().green(), &username.green());
                let project_dirs = ProjectDirs::from("", "", "Customs")
                    .expect("Failed to get project directories");
                let data_dir_path = project_dirs.data_dir();

                let file_path = data_dir_path.join("accounts_info.json");

                if !file_path.exists() {
                    process::exit(0);
                }

                match fs::remove_file(file_path) {
                    Ok(()) => {}
                    Err(err) => {
                        println!("{} Could not delete file:\n{}", "ERROR".bold().red(), err)
                    }
                }
                process::exit(0);
            } else {
                // Status code is not 200, handle error
                match response.json::<serde_json::Value>().await {
                    Ok(json) => {
                        if let Some(error_message) = json.get("error") {
                            println!("{} {}", "ERROR".bold().red(), error_message);
                            process::exit(0);
                        } else {
                            println!("{} Unknown error occurred", "ERROR".bold().red());
                            process::exit(0);
                        }
                    }
                    Err(e) => {
                        println!("{} Parsing JSON response: {}", "ERROR".bold().red(), e);
                    }
                }
            }
        }
        Err(e) => {
            println!("{} Failed to send request: {}", "ERROR".red().bold(), e);
        }
    }
}
