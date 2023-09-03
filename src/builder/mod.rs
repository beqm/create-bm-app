use crate::prompt::Inputs;
use anyhow::{Context, Ok, Result};
use std::fs;

pub fn create(inputs: Inputs) -> Result<()> {
    Ok(())
}

pub fn create_dir(inputs: Inputs) -> Result<()> {
    if inputs.directory.as_path().exists() {
        println!("Directory already exists, please change the project name.");
        std::process::exit(0);
    }
    fs::create_dir(inputs.directory).context("Could not create directory to project.")?;
    Ok(())
}

pub fn build() {}
