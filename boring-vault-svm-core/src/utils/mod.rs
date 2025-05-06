pub mod bindings;
pub mod constants;
pub mod discriminator;
pub mod pdas;

use eyre::{Context, Result};
use std::fs;

pub use bindings::*;
pub use constants::*;
pub use discriminator::*;
pub use pdas::*;

pub fn load_json(file_path: &str) -> Result<serde_json::Value> {
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path))?;
    let json_value: serde_json::Value = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON from: {}", file_path))?;

    Ok(json_value)
}

pub fn get_value(json_value: &serde_json::Value, key: &str) -> Result<serde_json::Value> {
    json_value
        .get(key)
        .cloned()
        .ok_or_else(|| eyre::eyre!("Key '{}' not found", key))
}
