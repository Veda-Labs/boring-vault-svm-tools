use anchor_lang::{
    system_program, AccountDeserialize, Discriminator, InstructionData, ToAccountMetas,
};
use eyre::Result;
use solana_client::rpc_client::RpcClient;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use solana_sdk::hash::hash;
use spl_associated_token_account::get_associated_token_address_with_program_id;
use spl_associated_token_account::ID as ASSOCIATED_TOKEN_PROGRAM_ID;
use spl_token_2022::ID as TOKEN_2022_PROGRAM_ID;

use crate::KeypairOrPublickey;
use crate::{
    manage_instructions::ExternalInstruction,
    utils::{
        boring_vault_svm, get_asset_data_pda, get_cpi_digest_pda, get_program_config_pda,
        get_vault_pda, get_vault_share_mint, get_vault_state_pda,
    },
};

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
    let exchange_rate_provider = exchange_rate_provider.unwrap_or(authority);
    let payout_address = payout_address.unwrap_or(authority);
    let platform_fee_bps = platform_fee_bps.unwrap_or(0);
    let performance_fee_bps = performance_fee_bps.unwrap_or(0);
    let withdraw_authority = withdraw_authority.unwrap_or_default();
    let strategist = strategist.unwrap_or(authority);

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

pub fn create_manage_instruction<T: ExternalInstruction>(
    client: &RpcClient,
    signer: &KeypairOrPublickey,
    authority: Option<&KeypairOrPublickey>,
    eix: T,
) -> Result<Vec<Instruction>> {
    let mut instructions = vec![];

    // Get the signer's public key
    let signer_pubkey = signer.pubkey();

    // Call get_cpi_digest with the signer's public key
    let (cpi_digest_pda, digest) = get_cpi_digest(
        &signer_pubkey, // Pass the public key reference
        eix.vault_id(),
        &eix.ix_program_id(), // Pass reference to the returned Pubkey
        eix.ix_data(),
        eix.ix_remaining_accounts(),
        eix.ix_operators(),
    )?;

    // If the authority pubkey is provided, then we have the ability to setup the CPI digest.
    // Check if the PDA exists.
    match client.get_account(&cpi_digest_pda) {
        Ok(_) => {
            // The account exists so nothing to do.
        }
        Err(_) => {
            // This is okay if the authority was provided.
            if let Some(authority) = authority {
                // Authority needs to be able to sign the initialize CPI digest instruction.
                // Check if the provided authority can sign.
                if !authority.can_sign() {
                    return Err(eyre::eyre!(
                        "Authority provided for initializing CPI digest must be a Keypair, not just a Pubkey"
                    ));
                }
                // Add initialize CPI digest to instructions.
                instructions.push(create_initialize_cpi_digest_instruction(
                    &authority.pubkey(), // Use authority's pubkey
                    eix.vault_id(),
                    &cpi_digest_pda, // <<< Pass reference
                    digest,
                    eix.ix_operators(),
                )?);
            } else {
                return Err(eyre::eyre!(
                    "CPI digest account does not exist, and no authority Keypair was provided to initialize it"
                ));
            }
        }
    };

    let vault_state_pda = get_vault_state_pda(eix.vault_id());
    let vault_account = get_vault_pda(eix.vault_id(), eix.sub_account());
    let mut accounts = vec![
        AccountMeta::new(signer.pubkey(), true), // Use signer's pubkey here too
        AccountMeta::new(vault_state_pda, false),
        AccountMeta::new(vault_account, false),
        AccountMeta::new_readonly(cpi_digest_pda, false),
        AccountMeta::new_readonly(eix.ix_program_id(), false),
    ];
    accounts.extend(eix.ix_remaining_accounts());

    let args = boring_vault_svm::types::ManageArgs {
        vault_id: eix.vault_id(),
        sub_account: eix.sub_account(),
        ix_data: eix.ix_data(),
    };

    let manage_ix_data = boring_vault_svm::client::args::Manage { args }.data();

    // Create the instruction
    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts,
        data: manage_ix_data,
    };

    instructions.push(instruction);

    Ok(instructions)
}

pub fn create_update_asset_data_instruction(
    signer: &Pubkey,
    vault_id: u64,
    mint: &Pubkey,
    allow_deposits: bool,
    allow_withdrawals: bool,
    share_premium_bps: u16,
    is_pegged_to_base_asset: bool,
    price_feed: &Pubkey,
    inverse_price_feed: bool,
    max_staleness: u64,
    min_samples: u32,
) -> Result<Instruction> {
    let vault_state_pda = get_vault_state_pda(vault_id);
    let asset_data_pda = get_asset_data_pda(vault_state_pda, *mint);

    let accounts = boring_vault_svm::client::accounts::UpdateAssetData {
        signer: *signer,
        boring_vault_state: vault_state_pda,
        system_program: system_program::ID,
        asset: *mint,
        asset_data: asset_data_pda,
    };

    let asset_data = boring_vault_svm::accounts::AssetData {
        allow_deposits,
        allow_withdrawals,
        share_premium_bps,
        is_pegged_to_base_asset,
        price_feed: *price_feed,
        inverse_price_feed,
        max_staleness,
        min_samples,
    };

    let args = boring_vault_svm::types::UpdateAssetDataArgs {
        vault_id,
        asset_data,
    };

    let update_asset_data_ix_data = boring_vault_svm::client::args::UpdateAssetData { args }.data();

    // Create the instruction.
    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts: accounts.to_account_metas(None),
        data: update_asset_data_ix_data,
    };

    Ok(instruction)
}

pub fn create_deposit_sol_instruction(
    signer: &Pubkey,
    vault_id: u64,
    user_pubkey: &Pubkey,
    deposit_amount: u64,
    min_mint_amount: u64,
) -> Result<Instruction> {
    let vault_state_pda = get_vault_state_pda(vault_id);
    let native_mint = Pubkey::new_from_array([0; 32]);
    let asset_data_pda = get_asset_data_pda(vault_state_pda, native_mint);
    let vault_pda = get_vault_pda(vault_id, 0); // NOTE need to actually read state to see what deposit sub account is
    let share_mint = get_vault_share_mint(vault_state_pda);
    let user_share_ata = get_associated_token_address_with_program_id(
        user_pubkey,
        &share_mint,
        &TOKEN_2022_PROGRAM_ID,
    );
    let accounts = boring_vault_svm::client::accounts::DepositSol {
        signer: *signer,
        token_program_2022: TOKEN_2022_PROGRAM_ID,
        system_program: system_program::ID,
        associated_token_program: ASSOCIATED_TOKEN_PROGRAM_ID,
        boring_vault_state: vault_state_pda,
        boring_vault: vault_pda,
        asset_data: asset_data_pda,
        share_mint,
        user_shares: user_share_ata,
        price_feed: Pubkey::default(),
    };

    let args = boring_vault_svm::types::DepositArgs {
        vault_id,
        deposit_amount,
        min_mint_amount,
    };

    let deposit_sol_ix_data = boring_vault_svm::client::args::DepositSol { args }.data();

    // Create the instruction.
    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts: accounts.to_account_metas(None),
        data: deposit_sol_ix_data,
    };

    Ok(instruction)
}

pub fn create_set_deposit_sub_account_instruction(
    signer: &Pubkey,
    vault_id: u64,
    new_sub_account: u8,
) -> Result<Instruction> {
    let vault_state_pda = get_vault_state_pda(vault_id);

    let accounts = boring_vault_svm::client::accounts::SetDepositSubAccount {
        signer: *signer,
        boring_vault_state: vault_state_pda,
    };

    let set_deposit_sub_account_ix_data = boring_vault_svm::client::args::SetDepositSubAccount {
        vault_id,
        new_sub_account,
    }
    .data();

    // Create the instruction
    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts: accounts.to_account_metas(None),
        data: set_deposit_sub_account_ix_data,
    };

    Ok(instruction)
}

pub fn create_set_withdraw_sub_account_instruction(
    signer: &Pubkey,
    vault_id: u64,
    new_sub_account: u8,
) -> Result<Instruction> {
    let vault_state_pda = get_vault_state_pda(vault_id);

    let accounts = boring_vault_svm::client::accounts::SetWithdrawSubAccount {
        signer: *signer,
        boring_vault_state: vault_state_pda,
    };

    let set_withdraw_sub_account_ix_data = boring_vault_svm::client::args::SetWithdrawSubAccount {
        vault_id,
        new_sub_account,
    }
    .data();

    // Create the instruction
    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts: accounts.to_account_metas(None),
        data: set_withdraw_sub_account_ix_data,
    };

    Ok(instruction)
}

pub fn create_initialize_cpi_digest_instruction(
    signer: &Pubkey,
    vault_id: u64,
    cpi_digest_pda: &Pubkey,
    digest: [u8; 32],
    operators: boring_vault_svm::types::Operators,
) -> Result<Instruction> {
    let vault_state_pda = get_vault_state_pda(vault_id);
    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new(vault_state_pda, false),
        AccountMeta::new(*cpi_digest_pda, false),
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

// Needs remaining accounts
fn get_cpi_digest(
    signer_pubkey: &Pubkey,
    vault_id: u64,
    ix_program_id: &Pubkey,
    ix_data: Vec<u8>,
    ix_remaining_accounts: Vec<AccountMeta>,
    operators: boring_vault_svm::types::Operators,
) -> Result<(Pubkey, [u8; 32])> {
    let mut hash_data: Vec<u8> = Vec::new();

    // Start hashing with the inner instruction's program ID
    hash_data.extend(ix_program_id.to_bytes());

    // --- Construct the combined account list to mimic on-chain context ---
    // Order: Implicit accounts (from ViewCpiDigest), Payer, Remaining Accounts
    // Note: This assumes a plausible order. Exact runtime reordering is complex to replicate.

    // 1. Implicit account from ViewCpiDigest context
    let implicit_ix_program_id_meta = AccountMeta {
        pubkey: *ix_program_id,
        is_signer: false,
        is_writable: false,
    };

    // 2. Transaction fee payer (signer)
    let signer_meta = AccountMeta {
        pubkey: *signer_pubkey,
        is_signer: true,   // Runtime marks fee payer as signer
        is_writable: true, // Runtime marks fee payer as writable
    };

    // 3. Combine the accounts
    let mut combined_accounts = vec![implicit_ix_program_id_meta, signer_meta];
    combined_accounts.extend(ix_remaining_accounts.iter().cloned()); // Use cloned accounts

    // --- Apply operators using the combined list ---
    for operator in &operators.operators {
        match operator {
            boring_vault_svm::types::Operator::Noop => {}
            boring_vault_svm::types::Operator::IngestInstruction(ix_index, length) => {
                let from = *ix_index as usize;
                let to = from + (*length as usize);
                if to > ix_data.len() {
                    return Err(eyre::eyre!(
                        "IngestInstruction bounds [{},{}] out of range for ix_data len {}",
                        from,
                        to,
                        ix_data.len()
                    ));
                }
                hash_data.extend_from_slice(&ix_data[from..to]);
            }
            boring_vault_svm::types::Operator::IngestAccount(account_index) => {
                let idx = *account_index as usize;
                if idx >= combined_accounts.len() {
                    return Err(eyre::eyre!(
                         "IngestAccount index {} out of bounds. Combined accounts len: {}. Accounts: {:?}",
                         idx, combined_accounts.len(), combined_accounts.iter().map(|a| a.pubkey).collect::<Vec<_>>()
                     ));
                }
                // Use the combined_accounts list for indexing
                let account = &combined_accounts[idx];
                hash_data.extend_from_slice(account.pubkey.as_ref());
                hash_data.push(account.is_signer as u8);
                hash_data.push(account.is_writable as u8);
            }
            boring_vault_svm::types::Operator::IngestInstructionDataSize => {
                hash_data.extend_from_slice(&(ix_data.len() as u64).to_le_bytes());
            }
        }
    }

    // Calculate the final digest
    let digest = hash(&hash_data).to_bytes();

    // Get the PDA for this digest
    let cpi_digest_pda = get_cpi_digest_pda(vault_id, digest);

    Ok((cpi_digest_pda, digest))
}

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
