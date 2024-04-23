use clap::Args;
use crossterm::style::Stylize;
use dialoguer::Input;
use git2::{IndexAddOption, Repository};
use std::fs::{self, File};

#[derive(Args)]
pub struct Cli {}

// TODO improve 
pub fn init(_args: Cli) {
    let theme = dialoguer::theme::ColorfulTheme::default();

    let plugin_name: String = Input::with_theme(&theme)
        .with_prompt("Plugin Name")
        .interact_text()
        .unwrap();

    let plugin_description: String = Input::with_theme(&theme)
        .with_prompt("Plugin Description")
        .default("".to_string())
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
    let toml_data = format!("[plugin]\nname = \"{plugin_name}\"\nversion = \"0.1.0\"\ndescription = \"{plugin_description}\"\nauthors = [\"{plugin_author}\"]");
    let toml_file_path = format!("{}/manifest.toml", plugin_dir);
    fs::write(&toml_file_path, toml_data).expect("Failed to write to manifest.toml");

    // Write to .gitignore
    let gitignore_data = "# Ignore target directory\n/target/";
    let gitignore_file_path = format!("{}/.gitignore", plugin_dir);
    fs::write(&gitignore_file_path, gitignore_data).expect("Failed to write to .gitignore");

    // Create src directory and index.luau file
    let src_dir_path = format!("{}/src", plugin_dir);
    fs::create_dir(&src_dir_path).expect("Failed to create src directory");
    let lua_file_path = format!("{}/main.luau", src_dir_path);
    File::create(&lua_file_path).expect("Failed to create index.lua");

    // Initialize git repository
    let repo = Repository::init(plugin_dir).expect("Failed to initialize git repository");
    let mut index = repo.index().unwrap();
    let oid = index.write_tree().unwrap();
    let sig = repo.signature().unwrap();
    let tree = repo.find_tree(oid).unwrap();

    index.add_all(&["."], IndexAddOption::DEFAULT, None).unwrap();
    index.write().unwrap();

    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        "initial commit",
        &tree,
        &[],
    ).unwrap();

    println!(
        "{init_message} {plugin_name}\n\n{warning}\n{learn_more}\n\nHave Fun Coding!",
        init_message = "Initialized".green(),
        warning = "Swiffty commands and events must be written in Luau. We do this to ensure the safety of our users.".italic().yellow(),
        learn_more = "Learn more here: https://example.com/".bold().yellow()
    );
}
