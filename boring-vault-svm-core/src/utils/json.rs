use eyre::{Context, Result};
use serde_json::Value;
use solana_pubkey::Pubkey;
use std::{fs, path::PathBuf};

pub fn load_json(file_path: &str) -> Result<serde_json::Value> {
    let path = PathBuf::from(file_path);

    let full_path = if path.is_absolute() {
        path
    } else {
        std::env::current_dir()
            .with_context(|| "Failed to get current directory")?
            .join(path)
    };

    println!("Loading JSON from: {}", full_path.display());

    // Rest of the code remains the same...
    let content = fs::read_to_string(&full_path)
        .with_context(|| format!("Failed to read file: {}", full_path.display()))?;

    let json_value: serde_json::Value = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON from: {}", full_path.display()))?;

    Ok(json_value)
}

pub fn get_value(json_value: &serde_json::Value, key: &str) -> Result<serde_json::Value> {
    json_value
        .get(key)
        .cloned()
        .ok_or_else(|| eyre::eyre!("Key '{}' not found", key))
}

pub fn parse_pubkey_from_value(val_node: &Value, key: &str) -> Result<Pubkey> {
    val_node[key]
        .as_str()
        .ok_or_else(|| eyre::eyre!("Invalid string for key: '{}'", key))?
        .parse()
        .map_err(|e| eyre::eyre!("Parse error: {}", e))
}

pub fn parse_pubkey_vec_from_value(val_node: &Value, key: &str) -> Result<Vec<Pubkey>> {
    val_node[key]
        .as_array()
        .ok_or_else(|| eyre::eyre!("Invalid array for key: '{}'", key))?
        .iter()
        .map(|s| {
            s.as_str()
                .ok_or_else(|| eyre::eyre!("Invalid string in array"))?
                .parse()
                .map_err(|e| eyre::eyre!("Parse error: {}", e))
        })
        .collect()
}

pub fn parse_u16_vec_from_value(val_node: &Value, key: &str) -> Result<Vec<u16>> {
    val_node[key]
        .as_array()
        .ok_or_else(|| eyre::eyre!("Invalid array for key: '{}'", key))?
        .iter()
        .map(|s| {
            s.as_u64()
                .map(|v| v as u16)
                .ok_or_else(|| eyre::eyre!("Invalid u64 in array"))
        })
        .collect()
}
