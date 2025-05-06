use eyre::{Context, Result};
use serde_json::Value;
use solana_pubkey::Pubkey;
use std::fs;

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
