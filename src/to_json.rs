use std::io::Write;
use serde::{Serialize};

/// Function to write JSON data to a file
pub fn write_json_to_file<T: Serialize>(data: &T, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json_string = serde_json::to_string_pretty(data)?;
    let mut json_file = std::fs::File::create(file_name)?;
    json_file.write_all(json_string.as_bytes())?;
    Ok(())
}
