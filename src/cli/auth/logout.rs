use crossterm::style::Stylize;
use directories::ProjectDirs;
use std::fs::{self};
use std::process;
#[tokio::main]
pub async fn logout() {
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
