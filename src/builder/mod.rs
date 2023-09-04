use crate::prompt::Inputs;
use anyhow::{Context, Ok, Result};
use std::fs;
use std::io;
use walkdir::WalkDir;

const BASE_DIRECTORY: &str = "templates/base/";

pub fn create(inputs: Inputs) -> Result<()> {
    create_dir(&inputs)?;
    build(inputs)?;

    Ok(())
}

pub fn create_dir(inputs: &Inputs) -> Result<()> {
    if inputs.directory.as_path().exists() {
        println!("Directory already exists, please change the project name.");
        std::process::exit(0);
    }

    println!("Creating directory..");
    fs::create_dir(&inputs.directory).context("Could not create directory to project.")?;
    Ok(())
}

pub fn build(inputs: Inputs) -> Result<()> {
    for entry in WalkDir::new(&BASE_DIRECTORY) {
        let entry = entry?;

        let source_path = entry.path();
        let relative_path = source_path.strip_prefix(&BASE_DIRECTORY).unwrap(); // Calculate the relative path
        let destination_path = fs::canonicalize(&inputs.directory)
            .unwrap_or_default()
            .join(relative_path);

        if entry.file_type().is_file() {
            // Copy regular files
            fs::copy(&source_path, &destination_path)?;
        } else if entry.file_type().is_dir() {
            // Create directories
            fs::create_dir_all(&destination_path)?;
        }
    }

    Ok(())
}
