use clap::Args;
use crossterm::style::Stylize;
use dialoguer::Input;
use serde_json::json;
use std::fs;
use std::io::prelude::*;
use std::process::Command;

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

    let plugin_dir = &plugin_name;

    // Create plugin directory
    fs::create_dir(plugin_dir).expect("Failed to create directory");

    // Write to plugin.toml
    let toml_data = format!(
        r#"
        [Plugin]
        name = "{}"
        version = "1.0"
        description = "{}"
        author = "{}"
        "#,
        plugin_name, plugin_description, plugin_author
    );
    let toml_file_path = format!("{}/manifest.toml", plugin_dir);
    fs::write(&toml_file_path, toml_data).expect("Failed to write to manifest.toml");

    // Write to .gitignore
    let gitignore_data = "# Ignore target directory\n/target/";
    let gitignore_file_path = format!("{}/.gitignore", plugin_dir);
    fs::write(&gitignore_file_path, gitignore_data).expect("Failed to write to .gitignore");

    // Create src directory and index.lua file
    let src_dir_path = format!("{}/src", plugin_dir);
    fs::create_dir(&src_dir_path).expect("Failed to create src directory");
    let lua_file_path = format!("{}/index.lua", src_dir_path);
    fs::File::create(&lua_file_path).expect("Failed to create index.lua");

    // Initialize git repository
    Command::new("git")
        .args(&["init", plugin_dir])
        .output()
        .expect("Failed to initialize git repository");

    println!(
        "{} {}\n\n{}\n{}\n\nHave Fun Coding!",
        "Initialized".green(),
        plugin_name,
        "Swiffty commands and events must be written in Lua. We do this to avoid malware infecting plugins.".italic().yellow(),
        "Learn more here: https://example.com/".bold().yellow()
    );
}
