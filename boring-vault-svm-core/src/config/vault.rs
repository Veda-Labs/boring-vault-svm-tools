use std::{fs, path::Path};

use eyre::Result;
use serde::Deserialize;
use solana_pubkey::Pubkey;

#[derive(Debug, Clone, Deserialize)]
pub struct VaultAssetConfig {
    pub mint: Pubkey,
    pub decimals: u8,
    pub is_token_2022: bool,
    #[serde(default)]
    pub oracle: Option<Pubkey>,
}

#[derive(Debug, Deserialize)]
pub struct VaultConfig {
    pub vault_id: u64,
    pub sub_accounts: Vec<u8>,
    pub assets: Vec<VaultAssetConfig>,
}

impl VaultConfig {
    pub fn new(json_path: &str) -> Result<Self> {
        let json_content = fs::read_to_string(Path::new(json_path))
            .map_err(|e| eyre::eyre!("Failed to read JSON file '{}': {}", json_path, e))?;

        let config: VaultConfig = serde_json::from_str(&json_content)
            .map_err(|e| eyre::eyre!("Failed to parse JSON file '{}': {}", json_path, e))?;

        Ok(config)
    }
}
