use crate::builder::utils::{create_project_dir, initialize_base, initialize_features};
use crate::prompt::Inputs;
use anyhow::{Ok, Result};
use colored::*;
use git2::Repository;
use std::time::Instant;

mod utils;

pub fn create(inputs: Inputs) -> Result<()> {
    create_project_dir(&inputs)?;
    build(&inputs)?;
    if inputs.repo {
        let start_time = Instant::now();
        let styled_start = format!("Initializing git repository...").white();
        println!("{styled_start}");
        Repository::init(&inputs.directory)?;
        let end_time = Instant::now();
        let elapsed_time = end_time - start_time;

        let styled = format!(
            "\u{2714} Completed git repository in {}ms\n",
            elapsed_time.as_millis()
        )
        .blue();
        println!("{styled}");
    }

    let styled = format!("\u{2714} Your project is ready!").green();
    println!("{styled}");
    Ok(())
}

pub fn build(inputs: &Inputs) -> Result<()> {
    let start_time = Instant::now();
    let styled_start = format!("\nBuilding base project...").white();
    println!("{styled_start}");
    initialize_base(&inputs)?;
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;

    let styled_end = format!(
        "\u{2714} Completed base in {}ms\n",
        elapsed_time.as_millis()
    )
    .blue();
    println!("{styled_end}");

    if !inputs.features.is_empty() {
        let start_time = Instant::now();
        let styled_start = format!("Building features...").white();
        println!("{styled_start}");
        initialize_features(&inputs)?;
        let end_time = Instant::now();
        let elapsed_time = end_time - start_time;

        let styled = format!(
            "\u{2714} Completed features in {}ms\n",
            elapsed_time.as_millis()
        )
        .blue();
        println!("{styled}");
    }

    Ok(())
}
