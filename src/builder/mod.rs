use crate::prompt::Inputs;
use anyhow::{Context, Ok, Result};
use std::fs;
use walkdir::WalkDir;

const BASE_DIRECTORY: &str = "templates/base/";
const FEATURES_DIRECTORY: &str = "templates/features/";

pub fn create(inputs: Inputs) -> Result<()> {
    create_dir(&inputs)?;
    build(inputs)?;

    println!("Your project is ready!");
    Ok(())
}

pub fn create_dir(inputs: &Inputs) -> Result<()> {
    if inputs.directory.as_path().exists() {
        println!("Directory already exists, please change the project name.");
        std::process::exit(0);
    }

    fs::create_dir(&inputs.directory).context("Could not create directory to project.")?;
    Ok(())
}

pub fn build(inputs: Inputs) -> Result<()> {
    for entry in WalkDir::new(&BASE_DIRECTORY) {
        let entry = entry?;

        let source_path = entry.path();
        let relative_path = source_path.strip_prefix(&BASE_DIRECTORY).unwrap();
        let destination_path = fs::canonicalize(&inputs.directory)
            .unwrap_or_default()
            .join(relative_path);

        if entry.file_type().is_file() {
            fs::copy(&source_path, &destination_path)?;
        } else if entry.file_type().is_dir() {
            fs::create_dir_all(&destination_path)?;
        }
    }

    if !inputs.features.is_empty() {
        for feature in inputs.features.iter() {
            for entry in WalkDir::new(&FEATURES_DIRECTORY) {
                let entry = entry?;
                if entry.file_name().to_str().unwrap().to_string() == feature.to_lowercase() {
                    println!(
                        "{} = {}: {}",
                        entry.file_name().to_str().unwrap().to_string(),
                        feature.to_lowercase(),
                        entry.file_name().to_str().unwrap().to_string() == feature.to_lowercase()
                    );
                    println!("{:?}", entry.file_name());
                    let source_path = entry.path();
                    let relative_path = source_path.strip_prefix(&FEATURES_DIRECTORY).unwrap();
                    let destination_path = fs::canonicalize(&inputs.directory)
                        .unwrap_or_default()
                        .join(relative_path);

                    if entry.file_type().is_file() {
                        fs::copy(&source_path, &destination_path)?;
                    } else if entry.file_type().is_dir() {
                        fs::create_dir_all(&destination_path)?;
                    }
                }
            }
        }
    }

    Ok(())
}
