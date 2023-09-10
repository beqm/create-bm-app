use crate::builder::utils::{create_project_dir, initialize_base, initialize_features};
use crate::prompt::Inputs;
use anyhow::{Ok, Result};

mod utils;

pub fn create(inputs: Inputs) -> Result<()> {
    create_project_dir(&inputs)?;
    build(inputs)?;

    println!("Your project is ready!");
    Ok(())
}

pub fn build(inputs: Inputs) -> Result<()> {
    initialize_base(&inputs)?;

    if !inputs.features.is_empty() {
        initialize_features(&inputs)?;
    }

    Ok(())
}
