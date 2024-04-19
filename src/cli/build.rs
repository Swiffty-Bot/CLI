use clap::Args;
use crossterm::style::Stylize;
use dialoguer::Confirm;
use git2::{Repository, StatusOptions};
use std::env;
use std::fs;
use std::io::{self, prelude::*};
use std::path::{Path, PathBuf};
use toml::Value;
use zip::write::FileOptions;
use zip::ZipWriter;

#[derive(Debug, serde::Deserialize)]
struct Plugin {
    name: String,
    description: String,
    version: String,
    author: String,
}

#[derive(Debug, serde::Deserialize)]
struct Manifest {
    Plugin: Plugin,
}

#[derive(Args)]
pub struct Cli {}

pub fn build(_args: Cli) {
    if let Some(current_dir) = env::current_dir().ok() {
        let manifest_path = current_dir.join("manifest.toml");

        // Check for manifest file
        if !manifest_path.exists() {
            println!(
                "{} {}",
                "Error:".bold().red(),
                "manifest.toml file not found".bold()
            );
            return;
        }

        // Check manifest file
        if !check_manifest(&manifest_path) {
            return;
        }

        // Check GitHub repository
        if !check_github_repo(&current_dir) {
            return;
        }

        // Check if /target folder exists
        let target_dir = current_dir.join("target");
        if !target_dir.exists() {
            // Create /target folder
            if let Err(err) = create_target_folder(&current_dir) {
                println!("{} {}", "Error:".bold().red(), err);
                return;
            }

            // Check if zip file already exists
            let toml_content =
                fs::read_to_string(&manifest_path).expect("Failed to read manifest file");
            let manifest =
                toml::from_str::<Manifest>(&toml_content).expect("Failed to parse manifest");
            let name = manifest.Plugin.name.clone();
            let version = manifest.Plugin.version.clone();
            if check_existing_zip(&name, &version, &target_dir) {
                // Zip file exists, prompt user
                if !handle_existing_zip(&name, &version) {
                    return;
                }
            }

            // Continue with creating the zip file
            let (ignore_dirs, _) = match find_gitignore(&current_dir) {
                Ok(ignore_dirs) => (ignore_dirs, vec![]),
                Err(e) => {
                    println!("{} {}", "Error:".bold().red(), e);
                    return;
                }
            };

            if let Err(err) = create_zip_archive(&current_dir, &target_dir, &ignore_dirs) {
                println!("{} {}", "Error:".bold().red(), err);
            }
        } else {
            println!(
                "{} {}",
                "Warning:".bold().yellow(),
                "The /target folder already exists. Skipping creation."
            );
        }
    } else {
        println!(
            "{} {}",
            "Error:".bold().red(),
            "Failed to get current directory".bold()
        );
    }
}

fn check_existing_zip(name: &str, version: &str, target_dir: &Path) -> bool {
    let expected_name = format!("{}@{}.zip", name, version);
    target_dir.join(&expected_name).exists()
}

fn create_zip_archive(
    current_dir: &Path,
    target_dir: &Path,
    ignore_dirs: &[&Path],
) -> Result<(), io::Error> {
    let manifest_path = current_dir.join("manifest.toml");
    let toml_content =
        fs::read_to_string(&manifest_path).expect("Failed to read manifest file");
    let manifest =
        toml::from_str::<Manifest>(&toml_content).expect("Failed to parse manifest");
    let name = &manifest.Plugin.name;
    let version = &manifest.Plugin.version;

    let zip_file_path = target_dir.join(format!("{}@{}.zip", name, version));
    let file = fs::File::create(&zip_file_path)?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    let mut walker = ignore::WalkBuilder::new(&current_dir).build();
    while let Some(entry) = walker.next() {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(&current_dir)?;

        if ignore_dirs.iter().any(|ignored| path.starts_with(ignored)) {
            continue; // Skip ignored directories
        }

        if path.is_dir() {
            zip.add_directory(relative_path.to_str().unwrap(), options)?;
        } else {
            let mut file = fs::File::open(&path)?;
            file.read_to_end(&mut buffer)?;
            zip.start_file(relative_path.to_str().unwrap(), options)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        }
    }

    zip.finish()?;
    println!(
        "{} {}",
        "Zip archive created at:".bold().green(),
        zip_file_path.display()
    );
    Ok(())
}

fn check_manifest(manifest_path: &Path) -> bool {
    if let Ok(manifest_content) = fs::read_to_string(manifest_path) {
        let toml: toml::Value = manifest_content.parse().unwrap();

        let plugin_section = toml.get("Plugin").and_then(|value| value.as_table());
        if let Some(plugin_section) = plugin_section {
            let required_fields = ["name", "description", "version", "author"];
            let mut missing_fields = false;

            for field in &required_fields {
                if let Some(value) = plugin_section.get(*field).and_then(|value| value.as_str()) {
                    println!("{}: {}", field, value);
                } else {
                    println!(
                        "{} '{} {}'",
                        "Error:".bold().red(),
                        field.bold(),
                        "field missing in manifest.toml".bold()
                    );
                    missing_fields = true;
                }
            }

            if !missing_fields {
                return true;
            }
        } else {
            println!(
                "{} {}",
                "Error:".bold().red(),
                "Failed to find [Plugin] section in manifest.toml".bold()
            );
        }
    } else {
        println!(
            "{} {}",
            "Error:".bold().red(),
            "Failed to read manifest.toml file".bold()
        );
    }
    false
}

fn check_github_repo(current_dir: &PathBuf) -> bool {
    if let Ok(repo) = Repository::open(current_dir) {
        // Check for uncommitted changes
        let mut status_opts = StatusOptions::new();
        status_opts.include_untracked(true);
        if let Ok(status) = repo.statuses(Some(&mut status_opts)) {
            if !status.is_empty() {
                println!(
                    "{} {}",
                    "Error:".bold().red(),
                    "Uncommitted changes in the Git repository".bold()
                );
                return false;
            }
        } else {
            println!(
                "{} {}",
                "Error:".bold().red(),
                "Failed to check Git repository status".bold()
            );
            return false;
        }
    } else {
        println!(
            "{} {}",
            "Error:".bold().red(),
            "Git repository not found in current directory".bold()
        );
        return false;
    }
    true
}

fn find_gitignore(current_dir: &PathBuf) -> Result<Vec<&Path>, String> {
    let gitignore_path = current_dir.join(".gitignore");
    if gitignore_path.exists() {
        if let Ok(contents) = fs::read_to_string(&gitignore_path) {
            let mut ignore_dirs = Vec::new();
            for line in contents.lines() {
                if !line.trim().is_empty() && !line.starts_with('#') {
                    let path = current_dir.join(line.trim());
                    if path.exists() {
                        ignore_dirs.push(path.as_path());
                    }
                }
            }
            Ok(ignore_dirs)
        } else {
            Err("Failed to read .gitignore file".to_string())
        }
    } else {
        Err("No .gitignore file found".to_string())
    }
}

fn create_target_folder(current_dir: &PathBuf) -> Result<(), String> {
    let target_dir = current_dir.join("target");
    if !target_dir.exists() {
        fs::create_dir(&target_dir)
            .map_err(|e| format!("Failed to create /target directory: {}", e))?;
    }
    Ok(())
}

fn handle_existing_zip(name: &str, version: &str) -> bool {
    let target_dir = Path::new("./target");
    if let Ok(entries) = std::fs::read_dir(target_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    let expected_name = format!("{}@{}.zip", name, version);
                    if file_name == expected_name {
                        let replace_prompt = Confirm::new()
                            .with_prompt(format!("Replace {}@{}.zip?", name, version))
                            .interact()
                            .unwrap();
                        if !replace_prompt {
                            println!("Build cancelled by user.");
                            return false;
                        }
                        // Delete existing zip file
                        let zip_file_path = target_dir.join(&expected_name);
                        if let Err(err) = std::fs::remove_file(&zip_file_path) {
                            println!("Failed to delete existing zip file: {}", err);
                            return false;
                        }
                    }
                }
            }
        }
    }
    true
}
