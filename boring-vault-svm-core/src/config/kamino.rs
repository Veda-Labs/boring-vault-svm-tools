use eyre::Result;
use serde_json::Value;
use solana_pubkey::Pubkey;

use crate::utils::{get_value, load_json};

use crate::utils::{
    parse_pubkey_from_value, parse_pubkey_vec_from_value, parse_u16_vec_from_value,
};

#[derive(Debug)]
pub struct KaminoLendStrategyConfig {
    pub reserve: Pubkey,
    pub reserve_farm_state: Pubkey,
    pub reserve_liquidity_mint: Pubkey,
    pub reserve_liquidity_supply: Pubkey,
    pub reserve_collateral_mint: Pubkey,
    pub reserve_destination_deposit_collateral: Pubkey,
}

impl KaminoLendStrategyConfig {
    fn from_json_obj(data: &Value) -> Result<Self> {
        Ok(Self {
            reserve: parse_pubkey_from_value(data, "reserve")?,
            reserve_farm_state: parse_pubkey_from_value(data, "reserve_farm_state")?,
            reserve_liquidity_mint: parse_pubkey_from_value(data, "reserve_liquidity_mint")?,
            reserve_liquidity_supply: parse_pubkey_from_value(data, "reserve_liquidity_supply")?,
            reserve_collateral_mint: parse_pubkey_from_value(data, "reserve_collateral_mint")?,
            reserve_destination_deposit_collateral: parse_pubkey_from_value(
                data,
                "reserve_destination_deposit_collateral",
            )?,
        })
    }
}

#[derive(Debug)]
pub struct KaminoBorrowStrategyConfig {
    pub reserve: Pubkey,
    pub reserve_source_liquidity_mint: Pubkey,
    pub reserve_source_liquidity: Pubkey,
    pub reserve_source_liquidity_fee_receiver: Pubkey,
}

impl KaminoBorrowStrategyConfig {
    fn from_json_obj(data: &Value) -> Result<Self> {
        Ok(Self {
            reserve: parse_pubkey_from_value(data, "reserve")?,
            reserve_source_liquidity_mint: parse_pubkey_from_value(
                data,
                "reserve_source_liquidity_mint",
            )?,
            reserve_source_liquidity: parse_pubkey_from_value(data, "reserve_source_liquidity")?,
            reserve_source_liquidity_fee_receiver: parse_pubkey_from_value(
                data,
                "reserve_source_liquidity_fee_receiver",
            )?,
        })
    }
}

// TODO: move these top level:
// lending_market, oracle prices
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
        let all_data = load_json(json_path)?;

        let lend_profile_base = get_value(&all_data, lend_strategy_profile_key).map_err(|_| {
            eyre::eyre!(
                "Failed to get lend profile key '{}' from {}",
                lend_strategy_profile_key,
                json_path
            )
        })?;
        let lend_data_obj = get_value(&lend_profile_base, "lend").map_err(|_| {
            eyre::eyre!(
                "Failed to get 'lend' sub-data for profile '{}' from {}",
                lend_strategy_profile_key,
                json_path
            )
        })?;

        let borrow_profile_base =
            get_value(&all_data, borrow_strategy_profile_key).map_err(|_| {
                eyre::eyre!(
                    "Failed to get borrow profile key '{}' from {}",
                    borrow_strategy_profile_key,
                    json_path
                )
            })?;
        let borrow_data_obj = get_value(&borrow_profile_base, "borrow").map_err(|_| {
            eyre::eyre!(
                "Failed to get 'borrow' sub-data for profile '{}' from {}",
                borrow_strategy_profile_key,
                json_path
            )
        })?;

        let lend = KaminoLendStrategyConfig::from_json_obj(&lend_data_obj)?;
        let borrow = KaminoBorrowStrategyConfig::from_json_obj(&borrow_data_obj)?;

        let lending_market = parse_pubkey_from_value(&lend_data_obj, "lending_market")?;
        let oracle_prices = parse_pubkey_from_value(&lend_data_obj, "oracle_prices")?;
        let oracle_mapping = parse_pubkey_from_value(&lend_data_obj, "oracle_mapping")?;
        let oracle_twaps = parse_pubkey_from_value(&lend_data_obj, "oracle_twaps")?;
        let price_accounts = parse_pubkey_vec_from_value(&lend_data_obj, "price_accounts")?;
        let tokens = parse_u16_vec_from_value(&lend_data_obj, "tokens")?;

        Ok(Self {
            lend,
            borrow,
            lending_market,
            oracle_prices,
            oracle_mapping,
            oracle_twaps,
            price_accounts,
            tokens,
        })
    }
}
