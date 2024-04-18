use clap::Args;
use crossterm::style::Stylize;
use dialoguer::Input;
use serde_json::json;
use std::io::prelude::*;
use std::fs;

#[derive(Args)]
pub struct Cli {}

pub fn init(_args: Cli) {
    let theme = dialoguer::theme::ColorfulTheme::default();

    let plugin_name: String = Input::with_theme(&theme)
        .with_prompt("Plugin Name")
        .interact_text()
        .unwrap();

    let plugin_description: String = Input::with_theme(&theme)
        .with_prompt("Plugin Description")
        .interact_text()
        .unwrap();

    let plugin_author: String = Input::with_theme(&theme)
        .with_prompt("Plugin Author")
        .interact_text()
        .unwrap();

    let file_name = format!("{}/plugin-manifest.json", plugin_name);

    let json_data = json!({
        "name": plugin_name,
        "version": "1.0",
        "description": plugin_description,
        "author": plugin_author
    });

    let formatted_json_data =
        serde_json::to_string_pretty(&json_data).expect("Failed to serialize JSON data");

    fs::create_dir(&plugin_name).expect("Failed to create directory");

    let mut file = fs::File::create(&file_name).expect("Failed to create file");
    file.write_all(formatted_json_data.as_bytes())
        .expect("Failed to write to file");

    println!("{} {}\n\n{}\n{}\n\nHave Fun Coding!", "Initialized".green(), plugin_name, "Swiffty commands and events must be writen in lua. We do this to avoid malwear infecting plugins.".italic().yellow(), "Learn more here: https://example.com/".bold().yellow());
}
