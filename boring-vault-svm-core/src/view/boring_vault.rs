use std::collections::HashMap;

use anchor_lang::AccountDeserialize;
use eyre::Result;
use solana_client::rpc_client::RpcClient;
use solana_pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address_with_program_id;

use crate::{
    config::VaultConfig,
    instructions::WSOL_MINT,
    utils::{
        boring_vault_svm::{
            self,
            accounts::{AssetData, BoringVault},
        },
        get_asset_data_pda, get_token_account_balance, get_vault_pda, get_vault_state_pda,
    },
};

pub fn get_asset_data(
    client: &RpcClient,
    vault_id: u64,
    mint: Pubkey,
) -> Result<(Pubkey, AssetData)> {
    let vault_state_pda = get_vault_state_pda(vault_id);
    let asset_data_pda = get_asset_data_pda(vault_state_pda, mint);

    let asset_data_bytes = client.get_account_data(&asset_data_pda)?;

    let asset_data =
        boring_vault_svm::accounts::AssetData::try_deserialize(&mut &asset_data_bytes[..])?;

    Ok((asset_data_pda, asset_data))
}

pub fn get_vault_state(client: &RpcClient, vault_id: u64) -> Result<(Pubkey, BoringVault)> {
    let vault_state_pda = get_vault_state_pda(vault_id);
    let vault_data_bytes = client.get_account_data(&vault_state_pda)?;

    let vault_data =
        boring_vault_svm::accounts::BoringVault::try_deserialize(&mut &vault_data_bytes[..])?;

    Ok((vault_state_pda, vault_data))
}

pub fn get_sub_account_token_totals(
    client: &RpcClient,
    vault_config: &VaultConfig,
) -> Result<HashMap<Pubkey, u64>> {
    let mut totals_per_mint: HashMap<Pubkey, u64> = HashMap::new();

    for sub_account_id in &vault_config.sub_accounts {
        let sub_account_pda = get_vault_pda(vault_config.vault_id, *sub_account_id);
        println!("Sub-account {}: {}", sub_account_id, sub_account_pda);

        // Check native SOL balance
        if let Ok(account) = client.get_account(&sub_account_pda) {
            let lamports = account.lamports;
            *totals_per_mint.entry(*WSOL_MINT).or_insert(0) += lamports;

            println!(
                "  Native SOL balance for sub-account {}: {} lamports ({} SOL)",
                sub_account_id,
                lamports,
                lamports as f64 / 1_000_000_000.0
            );
        }

        for asset_config in &vault_config.assets {
            let mint_pubkey = asset_config.mint;

            if let Some(amount) = get_token_account_balance(
                client,
                &sub_account_pda,
                &mint_pubkey,
                asset_config.is_token_2022,
            )? {
                *totals_per_mint.entry(mint_pubkey).or_insert(0) += amount;

                let human_amount = amount as f64 / 10f64.powi(asset_config.decimals as i32);

                println!(
                    "  Found {} for mint {} in sub-account {} ata {}: {} raw units ({} tokens)",
                    if asset_config.is_token_2022 {
                        "Token-2022"
                    } else {
                        "SPL Token"
                    },
                    mint_pubkey,
                    sub_account_id,
                    get_associated_token_address_with_program_id(
                        &sub_account_pda,
                        &mint_pubkey,
                        if asset_config.is_token_2022 {
                            &spl_token_2022::ID
                        } else {
                            &spl_token::ID
                        }
                    ),
                    amount,
                    human_amount
                );
            }
        }
    }
    Ok(totals_per_mint)
}
