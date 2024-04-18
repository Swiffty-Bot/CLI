use clap::Args;
use crossterm::style::Stylize;
use serde_json::Value;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read};
use std::process;
use walkdir::WalkDir;
use zip::{CompressionMethod, ZipWriter};

#[derive(Args)]
pub struct Cli {}

pub fn build(_args: Cli) {
    // Read the manifest file
    let manifest_path = "manifest.json";
    let manifest_content = match fs::read_to_string(manifest_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("{} {}", "Error reading manifest file:".bold().red(), err);
            process::exit(1);
        }
    };

    // Parse the manifest content
    let mut manifest_json: Value = match serde_json::from_str(&manifest_content) {
        Ok(json) => json,
        Err(err) => {
            eprintln!("{} {}", "Error parsing manifest file:".bold().red(), err);
            process::exit(1);
        }
    };

    // Extract name from the manifest and ensure it only contains alphanumeric characters
    let name = match manifest_json["name"].as_str() {
        Some(name)
            if !name.trim().is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric()) =>
        {
            name
        }
        _ => {
            eprintln!("{}", "Error: 'name' field is empty, not found, or contains non-alphanumeric characters in manifest file".bold().red());
            process::exit(1);
        }
    };

    // Extract version from the manifest and validate it as semantic version
    let version = match manifest_json["version"].as_str() {
        Some(version) if is_semantic_version(version) => version,
        _ => {
            eprintln!("{}", "Error: 'version' field is empty, not found, or not a valid semantic version in manifest file".bold().red());
            process::exit(1);
        }
    };

    // Create the build directory if it doesn't exist
    if let Err(err) = fs::create_dir_all("build") {
        eprintln!("{} {}", "Error creating build directory:".bold().red(), err);
        process::exit(1);
    }

    // Delete existing .zip file if present (ignoring the version part)
    let existing_zip_path = format!("build/{}@*.zip", name);
    if let Err(err) = glob::glob(&existing_zip_path) {
        eprintln!(
            "{} {}",
            "Error deleting existing zip file:".bold().red(),
            err
        );
        process::exit(1);
    } else {
        for entry in glob::glob(&existing_zip_path).expect("Failed to read glob pattern") {
            if let Ok(path) = entry {
                if let Err(err) = fs::remove_file(&path) {
                    eprintln!(
                        "{} {}",
                        "Error deleting existing zip file:".bold().red(),
                        err
                    );
                    process::exit(1);
                }
            }
        }
    }

    // Create the zip file inside the build directory using the name and version from the manifest
    let zip_path = format!("build/{}@{}.zip", name, version);
    if let Err(err) = create_zip_archive(&zip_path, ".", &manifest_json) {
        eprintln!("{} {}", "Error creating zip archive:".bold().red(), err);
        process::exit(1);
    }

    println!("{}", "Build complete.".green().bold());
}

fn create_zip_archive(zip_path: &str, source_dir: &str, manifest: &Value) -> io::Result<()> {
    // Open a file to write the zip archive to
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(zip_path)?;
    // Create a ZipWriter to write the zip archive
    let mut zip_writer = ZipWriter::new(file);

    // Walk the source directory and add all files and directories to the zip archive
    let walker = WalkDir::new(source_dir);
    for entry in walker {
        let entry = entry?;
        let path = entry.path();
        let name = path.strip_prefix(source_dir).unwrap(); // Strip the leading source directory from the file path
        if path.is_file() {
            let options =
                zip::write::FileOptions::default().compression_method(CompressionMethod::Stored);
            zip_writer.start_file(name.to_string_lossy(), options)?;
            let mut file = File::open(path)?;
            io::copy(&mut file, &mut zip_writer)?;
        } else if path.is_dir() {
            let options =
                zip::write::FileOptions::default().compression_method(CompressionMethod::Stored);
            zip_writer.add_directory(name.to_string_lossy(), options)?;
        }
    }

    // Finish writing the zip archive
    zip_writer.finish()?;
    Ok(())
}

fn is_semantic_version(version: &str) -> bool {
    let re = regex::Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
    re.is_match(version)
}
