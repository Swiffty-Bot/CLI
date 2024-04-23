use clap::Args;
use crossterm::style::Stylize;
use dialoguer::Confirm;
use git2::{Repository, StatusOptions};
use semver::{Version, VersionReq};
use tracing::error;
use std::{
    collections::HashMap, env, fs::{self, File}, io::Write, path::PathBuf
};
use walkdir::WalkDir;
use zip::{write::FileOptions, ZipWriter};
use serde::{Serialize, Deserialize};

#[derive(Args)]
pub struct Cli {
    #[arg(long)]
    pub ignore_dirty: bool,
}

#[derive(Serialize, Deserialize)]
struct Manifest {
    pub name: String,
    pub version: Version, // enforces that it follows a version syntax
    pub target: PathBuf,
    pub dependencies: HashMap<String, VersionReq>, // version requirement syntax (ex: >=1.0.4). might remove this one if we don't want to do dependencies.
}

pub fn build(args: Cli) {
    if let Ok(current_dir) = env::current_dir() {
        let manifest_path = current_dir.join("manifest.toml");

        if !manifest_path.exists() {
            error!("manifest.toml file not found");
            return;
        }

        let manifest = check_manifest(&manifest_path);
        if let Err(err) = manifest {
            error!("Invalid manifest: {err}");
            return;
        }
        
        let manifest = manifest.unwrap();

        let repo = Repository::open(&current_dir);

        if repo.is_err() {
            error!("Git repository not found");
            return;
        }

        let repo = repo.unwrap();

        if !args.ignore_dirty && check_dirty(&repo) {
            return;
        }

        let target_dir = current_dir.join("target");
        if !target_dir.exists() {
            if let Err(err) = fs::create_dir(&target_dir) {
                error!("Failed to create /target directory: {}", err);
                return;
            }
        }

        let filename = format!("{}@{}.zip", manifest.name, manifest.version);
        let file_path = target_dir.join(&filename);

        if !check_existing_zip(&file_path) {
            return error!("Build canceled");
        }

        let valid_dirs = get_valid_dirs(&current_dir, &repo);

        if let Err(err) = create_zip(&file_path, &valid_dirs) {
            error!("Failed to create zip: {}", err);
        }

        println!(
            "{}",
            format!("Successfully created zip file at {}", file_path.display())
                .bold()
                .green()
        );
    }
}

// lazy here with the error, typically should use the `thiserror` crate and create a union type but since its only being called once ig its ok.
fn check_manifest(manifest_path: &PathBuf) -> Result<Manifest, Box<dyn std::error::Error>> {
    let mfdata = fs::read_to_string(manifest_path)?;
    Ok(toml::from_str(&mfdata)?)
}

fn check_dirty(repo: &Repository) -> bool {
    let mut status_opts = StatusOptions::new();
    status_opts.include_untracked(true);
    if let Ok(status) = repo.statuses(Some(&mut status_opts)) {
        if !status.is_empty() {
            error!("Uncommitted changes in the Git repository");
            return true;
        }
    } else {
        error!("Failed to check Git repository status");
        return true;
    }
    false
}

fn check_existing_zip(file_path: &PathBuf) -> bool {
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
fn get_valid_dirs(current_dir: &PathBuf, repo: &Repository) -> Vec<PathBuf> {
    WalkDir::new(current_dir)
        .into_iter()
        .filter_map(|entry| {
            let path = entry.ok()?.path().to_path_buf();
            if repo.is_path_ignored(&path).ok()? {
                return None;
            }

            Some(path)
        })
        .collect()
}

// because this might be made into a helper function, consider making an error type for it.
fn create_zip(
    file_path: &PathBuf,
    valid_dirs: &[PathBuf],
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(file_path)?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored);

    for path in valid_dirs {
        let data = fs::read(path)?;
        zip.start_file(path.display().to_string(), options)?;
        zip.write_all(&data)?;
    }

    zip.finish()?;
    Ok(())
}
