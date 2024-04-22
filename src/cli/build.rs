use clap::Args;
use crossterm::style::Stylize;
use dialoguer::Confirm;
use git2::{Repository, StatusOptions};
use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};
use toml::Value;
use walkdir::WalkDir;
use zip::{write::FileOptions, ZipWriter};

#[derive(Args)]
pub struct Cli {}

pub fn build(_args: Cli) {
    if let Ok(current_dir) = env::current_dir() {
        let manifest_path = current_dir.join("manifest.toml");

        if !manifest_path.exists() {
            error_message("manifest.toml file not found");
            return;
        }

        if !check_manifest(&manifest_path) {
            return;
        }

        if !check_github_repo(&current_dir) {
            return;
        }

        let target_dir = current_dir.join("target");
        if !target_dir.exists() {
            if let Err(err) = fs::create_dir(&target_dir) {
                error_message(&format!("Failed to create /target directory: {}", err));
                return;
            }
        }

        let (name, version) = match get_plugin_info(&manifest_path) {
            Some(info) => info,
            None => {
                error_message("Failed to retrieve plugin name and version from manifest.toml");
                return;
            }
        };

        let filename = format!("{}@{}.zip", name, version);
        let file_path = target_dir.join(&filename);

        if !check_existing_zip(&file_path) {
            return error_message("Build canceled");
        }

        let ignored_dirs = get_ignored_dirs(&current_dir);

        if let Err(err) = create_zip(&current_dir, &file_path, &ignored_dirs) {
            error_message(&format!("Failed to create zip: {}", err));
        }

        println!(
            "{}",
            format!("Successfully created zip file at {}", file_path.display())
                .bold()
                .green()
        );
    }
}

fn check_manifest(manifest_path: &Path) -> bool {
    if let Ok(manifest_content) = fs::read_to_string(manifest_path) {
        let toml: toml::Value = match manifest_content.parse() {
            Ok(value) => value,
            Err(err) => {
                error_message(&format!("Failed to parse manifest.toml: {}", err));
                return false;
            }
        };

        let plugin_section = match toml.get("Plugin").and_then(|value| value.as_table()) {
            Some(section) => section,
            None => {
                error_message("Failed to find [Plugin] section in manifest.toml");
                return false;
            }
        };

        let required_fields = ["name", "description", "version", "author"];
        let mut missing_fields = false;

        for field in &required_fields {
            if let Some(value) = plugin_section.get(*field).and_then(|value| value.as_str()) {
                println!("{}: {}", field, value);
            } else {
                error_message(&format!(
                    "Missing '{} {}' field in manifest.toml",
                    field.bold().white(),
                    "Error:".bold().red()
                ));
                missing_fields = true;
            }
        }

        let name = match plugin_section.get("name").and_then(|value| value.as_str()) {
            Some(name) => name,
            None => {
                error_message(&format!(
                    "Missing '{} {}' field in manifest.toml",
                    "name".bold().white(),
                    "Error:".bold().red()
                ));
                return false;
            }
        };

        if !name.chars().all(|c| c.is_alphabetic()) {
            error_message(&format!(
                "Invalid plugin name '{}', must contain only letters",
                name
            ));
            return false;
        }

        let version = match plugin_section
            .get("version")
            .and_then(|value| value.as_str())
        {
            Some(version) => version,
            None => {
                error_message(&format!(
                    "Missing '{} {}' field in manifest.toml",
                    "version".bold().white(),
                    "Error:".bold().red()
                ));
                return false;
            }
        };

        if !semver::Version::parse(version).is_ok() {
            error_message(&format!(
                "Invalid version '{}', must follow semantic versioning rules",
                version
            ));
            return false;
        }

        !missing_fields
    } else {
        error_message("Failed to read manifest.toml file");
        false
    }
}

fn check_github_repo(current_dir: &Path) -> bool {
    if let Ok(repo) = Repository::open(current_dir) {
        let mut status_opts = StatusOptions::new();
        status_opts.include_untracked(true);
        if let Ok(status) = repo.statuses(Some(&mut status_opts)) {
            if !status.is_empty() {
                error_message("Uncommitted changes in the Git repository");
                return false;
            }
        } else {
            error_message("Failed to check Git repository status");
            return false;
        }
    } else {
        error_message("Git repository not found in current directory");
        return false;
    }
    true
}

fn get_plugin_info(manifest_path: &Path) -> Option<(String, String)> {
    if let Ok(manifest_content) = fs::read_to_string(manifest_path) {
        if let Ok(toml) = manifest_content.parse::<Value>() {
            if let Some(plugin_section) = toml.get("Plugin").and_then(|value| value.as_table()) {
                let name = plugin_section
                    .get("name")
                    .and_then(|value| value.as_str())
                    .map(|name| name.to_string());

                let version = plugin_section
                    .get("version")
                    .and_then(|value| value.as_str())
                    .map(|version| version.to_string());

                return name.and_then(|name| version.map(|version| (name, version)));
            }
        }
    }
    None
}

fn check_existing_zip(file_path: &Path) -> bool {
    if file_path.exists() {
        let theme = dialoguer::theme::ColorfulTheme::default();
        let confirm = Confirm::with_theme(&theme);
        let result = confirm
            .with_prompt("Zip file already exists. Overwrite?")
            .interact()
            .unwrap();
        if result {
            return true;
        } else {
            return false;
        }
    }
    true
}

// does not actually support the full gitignore syntax (regexes, **/path, etc)
fn get_ignored_dirs(current_dir: &Path) -> Vec<String> {
    let mut ignored_dirs = Vec::new();
    if let Ok(ignore_content) = fs::read_to_string(current_dir.join(".gitignore")) {
        for line in ignore_content.lines() {
            if !line.trim().is_empty() && !line.starts_with('#') {
                ignored_dirs.push(line.trim().to_string());
            }
        }
    }
    ignored_dirs
}

fn create_zip(
    source_dir: &Path,
    file_path: &Path,
    ignored_dirs: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(file_path)?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored);

    for entry in WalkDir::new(source_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let rel_path = path.strip_prefix(source_dir)?;
        let rel_path_str = rel_path.to_string_lossy().to_string();

        if !ignored_dirs.iter().any(|dir| rel_path_str.starts_with(dir)) {
            if path.is_file() {
                zip.start_file(rel_path_str, options)?;
                let mut f = File::open(path)?;
                let mut buf = Vec::new();

                f.read(&mut buf)?;

                zip.write(&buf)?;
            } else if path.is_dir() {
                zip.add_directory(rel_path_str, options)?;
            }
        }
    }

    zip.finish()?;
    Ok(())
}

fn error_message(message: &str) {
    println!("{} {}", "Error:".bold().red(), message.bold().white());
}
