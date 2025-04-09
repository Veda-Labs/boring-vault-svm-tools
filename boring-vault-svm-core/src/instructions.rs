use crate::utils::bindings::boring_vault_svm;
use crate::utils::pdas::*;
use anchor_lang::AccountDeserialize;
use anchor_lang::{Discriminator, InstructionData};
use eyre::Result;
use solana_address_lookup_table_interface::instruction::create_lookup_table;
use solana_client::rpc_client::RpcClient;
use solana_instruction::account_meta::AccountMeta;
use solana_instruction::Instruction;
use solana_program::system_program;
use solana_program::sysvar::rent::ID as RENT_ID;
use solana_pubkey::Pubkey;
use solana_transaction::Transaction;
use spl_token_2022::ID as TOKEN_2022_PROGRAM_ID;

pub fn create_lut_instruction_return(
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

pub fn create_initialize_instruction(authority: &Pubkey, signer: &Pubkey) -> Result<Instruction> {
    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(boring_vault_svm::ID, true),
        AccountMeta::new(get_program_config_pda(), false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];

    let discriminator = boring_vault_svm::client::args::Initialize::DISCRIMINATOR;
    let mut ix_data = discriminator.to_vec();
    ix_data.extend_from_slice(&authority.to_bytes());

    // Create the instruction
    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts,
        data: ix_data,
    };

    Ok(instruction)
}

// TODO maybe this should take an optional vault_id, that way if you are deploying multiple in a bundle it works just fine!
pub fn create_deploy_instruction(
    client: &RpcClient,
    authority: &Pubkey,
    signer: &Pubkey,
    base_asset: &Pubkey,
    name: String,
    symbol: String,
    exchange_rate_provider: Option<Pubkey>,
    exchange_rate: u64,
    payout_address: Option<Pubkey>,
    allowed_exchange_rate_change_upper_bound: u16,
    allowed_exchange_rate_change_lower_bound: u16,
    minimum_update_delay_in_seconds: u32,
    platform_fee_bps: Option<u16>,
    performance_fee_bps: Option<u16>,
    withdraw_authority: Option<Pubkey>,
    strategist: Option<Pubkey>,
) -> Result<Instruction> {
    let vault_id = get_vault_id(client)?;
    let vault_state_pda = get_vault_state_pda(vault_id);
    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(get_program_config_pda(), false),
        AccountMeta::new(vault_state_pda, false),
        AccountMeta::new(get_vault_share_mint(vault_state_pda), false),
        AccountMeta::new_readonly(*base_asset, false),
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new_readonly(TOKEN_2022_PROGRAM_ID, false),
    ];

    let authority = *authority;
    let exchange_rate_provider = exchange_rate_provider.unwrap_or_else(|| authority);
    let payout_address = payout_address.unwrap_or_else(|| authority);
    let platform_fee_bps = platform_fee_bps.unwrap_or_else(|| 0);
    let performance_fee_bps = performance_fee_bps.unwrap_or_else(|| 0);
    let withdraw_authority = withdraw_authority.unwrap_or_default();
    let strategist = strategist.unwrap_or_else(|| authority);

    let args = boring_vault_svm::types::DeployArgs {
        authority,
        name,
        symbol,
        exchange_rate_provider,
        exchange_rate,
        payout_address,
        allowed_exchange_rate_change_upper_bound,
        allowed_exchange_rate_change_lower_bound,
        minimum_update_delay_in_seconds,
        platform_fee_bps,
        performance_fee_bps,
        withdraw_authority,
        strategist,
    };
    let deploy_ix_data = boring_vault_svm::client::args::Deploy { args }.data();

    // Create the instruction
    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts,
        data: deploy_ix_data,
    };

    Ok(instruction)
}

pub fn create_manage_instruction(
    signer: &Pubkey,
    vault_account: Pubkey,
    cpi_digest_pda: Pubkey,
    vault_id: u64,
    sub_account: u8,
    ix_program_id: Pubkey,
    ix_data: Vec<u8>,
) -> Result<Instruction> {
    let vault_state_pda = get_vault_state_pda(vault_id);
    // Derive vault_account metadata pda.
    let (user_metadata_pda, _) =
        Pubkey::find_program_address(&[b"user_meta", vault_account.as_ref()], &ix_program_id);
    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(vault_state_pda, false),
        AccountMeta::new(vault_account, false),
        AccountMeta::new_readonly(cpi_digest_pda, false),
        // Add remaining accounts
        AccountMeta::new(vault_account, false),
        AccountMeta::new(vault_account, false),
        AccountMeta::new(user_metadata_pda, false),
        AccountMeta::new_readonly(ix_program_id, false),
        AccountMeta::new_readonly(RENT_ID, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];

    let args = boring_vault_svm::types::ManageArgs {
        vault_id,
        sub_account,
        ix_program_id,
        ix_data,
    };

    let manage_ix_data = boring_vault_svm::client::args::Manage { args }.data();

    // Create the instruction
    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts,
        data: manage_ix_data,
    };

    Ok(instruction)
}

pub fn create_initialize_cpi_digest_instruction(
    client: &RpcClient,
    signer: &Pubkey,
    vault_id: u64,
    ix_program_id: Pubkey,
    ix_data: Vec<u8>,
    operators: boring_vault_svm::types::Operators,
) -> Result<Instruction> {
    let (cpi_digest_pda, digest) =
        get_cpi_digest(client, vault_id, ix_program_id, ix_data, operators.clone())?;

    println!("digest pda: {}", cpi_digest_pda);

    let vault_state_pda = get_vault_state_pda(vault_id);
    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new(vault_state_pda, false),
        AccountMeta::new(cpi_digest_pda, false),
    ];

    let args = boring_vault_svm::types::CpiDigestArgs {
        vault_id,
        cpi_digest: digest,
        operators,
    };

    let initialize_cpi_digest_ix_data =
        boring_vault_svm::client::args::InitializeCpiDigest { args }.data();

    // Create the instruction
    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts,
        data: initialize_cpi_digest_ix_data,
    };

    Ok(instruction)
}

fn get_cpi_digest(
    client: &RpcClient,
    _vault_id: u64,
    ix_program_id: Pubkey,
    ix_data: Vec<u8>,
    operators: boring_vault_svm::types::Operators,
) -> Result<(Pubkey, [u8; 32])> {
    let accounts = vec![AccountMeta::new_readonly(system_program::ID, false)];

    let args = boring_vault_svm::types::ViewCpiDigestArgs {
        ix_program_id: ix_program_id,
        ix_data: ix_data,
        operators: operators,
    };

    let view_cpi_digest_ix_data = boring_vault_svm::client::args::ViewCpiDigest { args }.data();

    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts,
        data: view_cpi_digest_ix_data,
    };

    // Create the transaction
    let transaction = Transaction::new_with_payer(&[instruction], None);
    let tx_res = client.simulate_transaction(&transaction)?;

    let digest = tx_res.value.return_data.unwrap().data.0;

    println!("View Cpi Digest called: {:?}", digest);

    todo!()

    // // Convert the digest to a fixed-size array and validate length
    // let digest: [u8; 32] = digest
    //     .try_into()
    //     .expect("Digest must be exactly 32 bytes long");

    // // Get the PDA for this digest
    // let cpi_digest_pda = get_cpi_digest_pda(vault_id, digest);

    // Ok((cpi_digest_pda, digest))
}

// TODO so if you are deploying multiple vaults in a bundle then this logic would be wrong.
fn get_vault_id(client: &RpcClient) -> Result<u64> {
    let program_config_pda = get_program_config_pda();
    match client.get_account(&program_config_pda) {
        Ok(account) => {
            let program_config =
                boring_vault_svm::accounts::ProgramConfig::try_deserialize(&mut &account.data[..])?;
            Ok(program_config.vault_count)
        }
        Err(_) => Ok(0), // Return 0 if account not found
    }
}
