use anyhow::Result;
use std::process::Command;

pub fn clear() -> Result<()> {
    Command::new("clear");
    Ok(())
}
