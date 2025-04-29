use super::bindings::boring_vault_svm;
use super::constants::*;
use anchor_lang::ToAccountMetas;
use eyre::Result;
use solana_address_lookup_table_interface::instruction::derive_lookup_table_address;
use solana_instruction::AccountMeta;
use solana_program::hash::hash;
use solana_pubkey::Pubkey;
use anchor_lang::AccountDeserialize;

pub fn get_lut_pda(authority: &Pubkey, recent_block_slot: u64) -> Pubkey {
    let res = derive_lookup_table_address(authority, recent_block_slot);
    res.0
}

pub fn get_cpi_digest_pda(vault_id: u64, digest: [u8; 32]) -> Pubkey {
    let (cpi_digest_pda, _) = Pubkey::find_program_address(
        &[b"cpi-digest", &vault_id.to_le_bytes()[..], &digest],
        &boring_vault_svm::ID,
    );
    cpi_digest_pda
}

pub fn get_program_config_pda() -> Pubkey {
    let (program_config, _) =
        Pubkey::find_program_address(&[BASE_SEED_CONFIG], &boring_vault_svm::ID);
    program_config
}

pub fn get_vault_state_pda(vault_id: u64) -> Pubkey {
    let (boring_vault_state, _) = Pubkey::find_program_address(
        &[b"boring-vault-state", &vault_id.to_le_bytes()[..]],
        &boring_vault_svm::ID,
    );
    boring_vault_state
}

pub fn get_vault_share_mint(vault_state_pda: Pubkey) -> Pubkey {
    let (share_mint, _) = Pubkey::find_program_address(
        &[b"share-token", vault_state_pda.as_ref()],
        &boring_vault_svm::ID,
    );
    share_mint
}

pub fn get_vault_pda(vault_id: u64, sub_account: u8) -> Pubkey {
    let (vault_pda, _) = Pubkey::find_program_address(
        &[b"boring-vault", &vault_id.to_le_bytes()[..], &[sub_account]],
        &boring_vault_svm::ID,
    );
    vault_pda
}

pub fn get_asset_data_pda(vault_state_pda: Pubkey, mint: Pubkey) -> Pubkey {
    let (asset_data_pda, _) = Pubkey::find_program_address(
        &[
            BASE_SEED_ASSET_DATA,
            vault_state_pda.as_ref(),
            mint.as_ref(),
        ],
        &boring_vault_svm::ID,
    );
    asset_data_pda
}

pub fn get_user_metadata_pda(user_pubkey: &Pubkey, program_id: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"user_meta", &user_pubkey.to_bytes()], program_id).0
}

pub fn get_obligation(
    tag: u8,
    id: u8,
    owner: &Pubkey,
    lending_market: &Pubkey,
    seed_1: &Pubkey,
    seed_2: &Pubkey,
    program_id: &Pubkey,
) -> Pubkey {
    Pubkey::find_program_address(
        &[
            &[tag],
            &[id],
            owner.as_ref(),
            lending_market.as_ref(),
            seed_1.as_ref(),
            seed_2.as_ref(),
        ],
        program_id,
    )
    .0
}

pub fn get_lending_market_authority(lending_market: &Pubkey, program_id: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"lma", lending_market.as_ref()], program_id).0
}