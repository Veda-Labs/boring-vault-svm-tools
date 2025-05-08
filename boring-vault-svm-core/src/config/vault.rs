use eyre::Result;
use solana_pubkey::Pubkey;
use std::str::FromStr;

use crate::utils::load_json;

#[derive(Debug, Clone)]
pub struct VaultAssetConfig {
    pub mint: Pubkey,
    pub decimals: u8,
    pub is_token_2022: bool,
}

#[derive(Debug)]
pub struct VaultConfig {
    pub vault_id: u64,
    pub sub_accounts: Vec<u8>,
    pub assets: Vec<VaultAssetConfig>,
}

impl VaultConfig {
    pub fn new(json_path: &str) -> Result<Self> {
        let config_data = load_json(json_path)?;

        let vault_id = config_data["vault_id"]
            .as_u64()
            .ok_or_else(|| eyre::eyre!("Missing or invalid 'vault_id' in {}", json_path))?;

        let sub_accounts: Vec<u8> = config_data["sub_accounts"]
            .as_array()
            .ok_or_else(|| eyre::eyre!("Missing or invalid 'sub_accounts' array in {}", json_path))?
            .iter()
            .map(|v| {
                v.as_u64()
                    .ok_or_else(|| eyre::eyre!("Invalid value in 'sub_accounts' array"))
                    .map(|sa| sa as u8)
            })
            .collect::<Result<Vec<u8>>>()
            .map_err(|e| eyre::eyre!("Failed to parse 'sub_accounts' in {}: {}", json_path, e))?;

        let assets_json = config_data["assets"]
            .as_array()
            .ok_or_else(|| eyre::eyre!("Missing or invalid 'assets' array in {}", json_path))?;

        let mut assets = Vec::new();
        for asset_val in assets_json {
            let mint_str = asset_val["mint"].as_str().ok_or_else(|| {
                eyre::eyre!(
                    "Missing or invalid 'mint' string in asset object in {}",
                    json_path
                )
            })?;
            let mint = Pubkey::from_str(mint_str).map_err(|e| {
                eyre::eyre!(
                    "Failed to parse mint string '{}' to Pubkey in {}: {}",
                    mint_str,
                    json_path,
                    e
                )
            })?;

            let decimals = asset_val["decimals"]
                .as_u64()
                .ok_or_else(|| {
                    eyre::eyre!(
                        "Missing or invalid 'decimals' in asset object in {}",
                        json_path
                    )
                })?
                .try_into()
                .map_err(|_| eyre::eyre!("Decimal value too large for u8 in {}", json_path))?;

            let is_token_2022 = asset_val["is_token_2022"]
                .as_bool()
                .ok_or_else(|| eyre::eyre!("missing token program"))?;

            assets.push(VaultAssetConfig {
                mint,
                decimals,
                is_token_2022,
            });
        }

        Ok(Self {
            vault_id,
            sub_accounts,
            assets,
        })
    }
}
