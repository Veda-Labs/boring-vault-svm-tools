use anchor_lang::pubkey;
use eyre::Result;
use solana_address_lookup_table_interface::instruction::create_lookup_table;
use solana_client::rpc_client::RpcClient;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;
use solana_sdk::system_instruction;
use spl_associated_token_account::{
    get_associated_token_address_with_program_id, instruction::create_associated_token_account,
};
use spl_token::ID as TOKEN_PROGRAM_ID;

use crate::{
    manage_instructions::{CloseAccount, TransferSol},
    utils::get_vault_pda,
    KeypairOrPublickey,
};

use super::create_manage_instruction;

pub fn create_lut_instruction(
    signer: &Pubkey,
    authority: &Pubkey,
    recent_slot: u64,
) -> Result<Instruction> {
    // Create the lookup table instruction
    let (lookup_table_ix, _) = create_lookup_table(
        *authority, // authority
        *signer,    // payer
        recent_slot,
    );

    Ok(lookup_table_ix)
}

pub fn create_account_with_seed_instruction(
    signer: &Pubkey,
    vault_id: u64,
    sub_account: u8,
    owning_program_id: &Pubkey,
) -> Result<Instruction> {
    let vault_pda = get_vault_pda(vault_id, sub_account);
    let pda = Pubkey::create_with_seed(
        &vault_pda,
        "4UpD2fh7xH3VP9QQaXtsS1YY3bxzWhtf",
        owning_program_id,
    )?;
    Ok(system_instruction::create_account_with_seed(
        signer,                             // from (funding account)
        &pda,                               // to (the account to create)
        &vault_pda,                         // base
        "4UpD2fh7xH3VP9QQaXtsS1YY3bxzWhtf", // seed string
        9938880,                            // amount of lamports to transfer to the created account
        1300,              // amount of space in bytes to allocate to the created account
        owning_program_id, // owner of the created account
    ))
}

pub fn create_associated_token_account_instruction(
    signer: &Pubkey,
    vault_id: u64,
    sub_account: u8,
    mint: &Pubkey,
    token_program: &Pubkey,
) -> Result<Instruction> {
    let vault_pda = get_vault_pda(vault_id, sub_account);

    // Create instruction to make an ATA for the vault_pda
    Ok(create_associated_token_account(
        signer,        // payer
        &vault_pda,    // wallet address that will own the ATA
        mint,          // token mint
        token_program, // token program ID
    ))
}

pub fn create_wrap_sol_instructions(
    client: &RpcClient,
    signer: &KeypairOrPublickey,
    authority: Option<&KeypairOrPublickey>,
    vault_id: u64,
    sub_account: u8,
    amount: u64,
) -> Result<Vec<Instruction>> {
    let mut instructions = vec![];

    let vault_pda = get_vault_pda(vault_id, sub_account);
    let w_sol_mint = pubkey!("So11111111111111111111111111111111111111112");
    let w_sol_ata =
        get_associated_token_address_with_program_id(&vault_pda, &w_sol_mint, &TOKEN_PROGRAM_ID);

    // Transfer amount to wSOL ata.
    let eix_0 = TransferSol::new(vault_id, sub_account, w_sol_ata, amount);
    instructions.extend(create_manage_instruction(client, signer, authority, eix_0)?);

    // Init ata if needed(which wraps it).
    if let Some(ix) = init_associated_token_account_if_needed(
        client,
        &signer.pubkey(),
        vault_id,
        sub_account,
        &w_sol_mint,
        &TOKEN_PROGRAM_ID,
    )? {
        instructions.push(ix);
    }

    Ok(instructions)
}

pub fn create_unwrap_sol_instructions(
    client: &RpcClient,
    signer: &KeypairOrPublickey,
    authority: Option<&KeypairOrPublickey>,
    vault_id: u64,
    sub_account: u8,
) -> Result<Vec<Instruction>> {
    let mut instructions = vec![];
    let vault_pda = get_vault_pda(vault_id, sub_account);
    let w_sol_mint = pubkey!("So11111111111111111111111111111111111111112");
    let w_sol_ata =
        get_associated_token_address_with_program_id(&vault_pda, &w_sol_mint, &TOKEN_PROGRAM_ID);

    let eix = CloseAccount::new(vault_id, sub_account, w_sol_ata, TOKEN_PROGRAM_ID);
    instructions.extend(create_manage_instruction(client, signer, authority, eix)?);

    Ok(instructions)
}

pub fn init_associated_token_account_if_needed(
    client: &RpcClient,
    signer: &Pubkey,
    vault_id: u64,
    sub_account: u8,
    mint: &Pubkey,
    token_program: &Pubkey,
) -> Result<Option<Instruction>> {
    let vault_pda = get_vault_pda(vault_id, sub_account);

    // Get the ATA address
    let ata = get_associated_token_address_with_program_id(&vault_pda, mint, token_program);

    // Check if the account exists
    match client.get_account(&ata) {
        Ok(_) => {
            // Account exists, return None
            Ok(None)
        }
        Err(_) => {
            // Account doesn't exist, create the instruction
            Ok(Some(create_associated_token_account(
                signer,        // payer
                &vault_pda,    // wallet address that will own the ATA
                mint,          // token mint
                token_program, // token program ID
            )))
        }
    }
}
