use std::fs;

use anyhow::Result;

pub fn read_file(path: String) -> Result<String> {
    let file_contents = fs::read_to_string(path)?;
    Ok(file_contents)
}
