use clap::Args;
use crossterm::style::Stylize;
use directories::ProjectDirs;
use serde_json::{json, Value};
use std::fs::{self, File, OpenOptions};
use std::io::Read;
use std::process;
use http::{Request, Response};

#[derive(Args)]
pub struct Cli {
    #[clap(short, long)]
    pub action: String,

    #[clap(short, long)]
    pub username: Option<String>,

    #[clap(short, long)]
    pub password: Option<String>,
}

pub fn account(args: Cli) {
    let action = &args.action.to_lowercase();

    match action.as_str() {
        "login" => login(args),
        "logout" => logout(),
        "create" => create(args),
        "delete" => delete(),
        "view" => view(),
        _ => {
            println!(
                "{} Action {} not supported",
                "ERROR".red(),
                &args.action.bold()
            );
            process::exit(1);
        }
    }
}

fn login(args: Cli) {
    let info = json!({
        "username": &args.username,
        "password": &args.password
    });


   

    let project_dirs =
        ProjectDirs::from("", "", "Customs").expect("Failed to get project directories");
    let data_dir_path = project_dirs.data_dir();

    if !data_dir_path.exists() {
        fs::create_dir_all(&data_dir_path).expect("Failed to create data directory");
    }

    let file_path = data_dir_path.join("accounts_info.json");

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&file_path)
        .expect("Failed to open file");

  
    serde_json::to_writer(file, &info).expect("Failed to write to file");

    println!("Login info saved successfully!");
}

fn logout() {
    let project_dirs = directories::ProjectDirs::from("", "", "Customs")
        .expect("Failed to get project directories");
    let data_dir_path = project_dirs.data_dir();
    let file_path = data_dir_path.join("accounts_info.json");

    if let Err(err) = fs::remove_file(&file_path) {
        println!("Failed to delete file: {}", err);
        return;
    }

    println!("File deleted successfully!");
}

fn create(args: Cli) {
    // Implement create functionality here
    unimplemented!();
}

fn delete() {
    // Implement delete functionality here
    unimplemented!();
}

fn view() {
    let project_dirs =
        directories::ProjectDirs::from("", "", "Customs").expect("Failed to get project directories");
    let data_dir_path = project_dirs.data_dir();
    let file_path = data_dir_path.join("accounts_info.json");

    let mut file = File::open(&file_path).expect("Failed to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let json_data: Value = serde_json::from_str(&contents).expect("Failed to parse JSON");

    println!("Contents of accounts_info.json:");
    println!("{}", json_data);
}
