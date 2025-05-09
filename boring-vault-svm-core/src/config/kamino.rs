use crate::utils::{deserialize_pubkey, deserialize_pubkey_vec};
use eyre::Result;
use serde::Deserialize;
use solana_pubkey::Pubkey;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct KaminoLendStrategyConfig {
    #[serde(deserialize_with = "deserialize_pubkey")]
    pub reserve: Pubkey,
    #[serde(deserialize_with = "deserialize_pubkey")]
    pub reserve_farm_state: Pubkey,
    #[serde(deserialize_with = "deserialize_pubkey")]
    pub reserve_liquidity_mint: Pubkey,
    #[serde(deserialize_with = "deserialize_pubkey")]
    pub reserve_liquidity_supply: Pubkey,
    #[serde(deserialize_with = "deserialize_pubkey")]
    pub reserve_collateral_mint: Pubkey,
    #[serde(deserialize_with = "deserialize_pubkey")]
    pub reserve_destination_deposit_collateral: Pubkey,
    #[serde(deserialize_with = "deserialize_pubkey")]
    pub lending_market: Pubkey,
    #[serde(deserialize_with = "deserialize_pubkey")]
    pub oracle_prices: Pubkey,
    #[serde(deserialize_with = "deserialize_pubkey")]
    pub oracle_mapping: Pubkey,
    #[serde(deserialize_with = "deserialize_pubkey")]
    pub oracle_twaps: Pubkey,
    #[serde(deserialize_with = "deserialize_pubkey_vec")]
    pub price_accounts: Vec<Pubkey>,
    pub tokens: Vec<u16>,
}

#[derive(Debug, Deserialize)]
pub struct KaminoBorrowStrategyConfig {
    #[serde(deserialize_with = "deserialize_pubkey")]
    pub reserve: Pubkey,
    #[serde(deserialize_with = "deserialize_pubkey")]
    pub reserve_source_liquidity_mint: Pubkey,
    #[serde(deserialize_with = "deserialize_pubkey")]
    pub reserve_source_liquidity: Pubkey,
    #[serde(deserialize_with = "deserialize_pubkey")]
    pub reserve_source_liquidity_fee_receiver: Pubkey,
}

#[derive(Debug)]
pub struct KaminoConfig {
    pub lend: KaminoLendStrategyConfig,
    pub borrow: KaminoBorrowStrategyConfig,
    pub lending_market: Pubkey,
    pub oracle_prices: Pubkey,
    pub oracle_mapping: Pubkey,
    pub oracle_twaps: Pubkey,
    pub price_accounts: Vec<Pubkey>,
    pub tokens: Vec<u16>,
}

impl KaminoConfig {
    pub fn new(
        json_path: &str,
        lend_strategy_profile_key: &str,
        borrow_strategy_profile_key: &str,
    ) -> Result<Self> {
        let json_content = fs::read_to_string(Path::new(json_path))
            .map_err(|e| eyre::eyre!("Failed to read JSON file '{}': {}", json_path, e))?;

        let all_data: serde_json::Value = serde_json::from_str(&json_content)
            .map_err(|e| eyre::eyre!("Failed to parse JSON file '{}': {}", json_path, e))?;

        let lend_config: KaminoLendStrategyConfig = serde_json::from_value(
            all_data[lend_strategy_profile_key]["lend"].clone(),
        )
        .map_err(|e| {
            eyre::eyre!(
                "Failed to parse lend profile '{}': {}",
                lend_strategy_profile_key,
                e
            )
        })?;

        let borrow_config: KaminoBorrowStrategyConfig =
            serde_json::from_value(all_data[borrow_strategy_profile_key]["borrow"].clone())
                .map_err(|e| {
                    eyre::eyre!(
                        "Failed to parse borrow profile '{}': {}",
                        borrow_strategy_profile_key,
                        e
                    )
                })?;

        Ok(Self {
            lend: lend_config.clone(),
            borrow: borrow_config,
            lending_market: lend_config.lending_market,
            oracle_prices: lend_config.oracle_prices,
            oracle_mapping: lend_config.oracle_mapping,
            oracle_twaps: lend_config.oracle_twaps,
            price_accounts: lend_config.price_accounts,
            tokens: lend_config.tokens,
        })
    }
}
