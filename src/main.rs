use anyhow::{Ok, Result};

mod builder;
mod prompt;
fn main() -> Result<()> {
    let inputs = prompt::get_inputs();
    builder::create(inputs)?;
    Ok(())
}
