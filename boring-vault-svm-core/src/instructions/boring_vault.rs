use anchor_lang::{
    system_program, AccountDeserialize, Discriminator, InstructionData, ToAccountMetas,
};
use eyre::Result;
use solana_client::rpc_client::RpcClient;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use solana_sdk::hash::hash;
use spl_associated_token_account::get_associated_token_address_with_program_id;
use spl_associated_token_account::instruction::create_associated_token_account;
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

pub fn create_pause_instruction(vault_id: u64, signer: &Pubkey) -> Result<Instruction> {
    let vault_state_pda = get_vault_state_pda(vault_id);

    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(vault_state_pda, false),
    ];

    let pause_ix_data = boring_vault_svm::client::args::Pause { vault_id }.data();

    // Create the instruction
    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts,
        data: pause_ix_data,
    };

    Ok(instruction)
}

pub fn create_unpause_instruction(vault_id: u64, signer: &Pubkey) -> Result<Instruction> {
    let vault_state_pda = get_vault_state_pda(vault_id);

    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(vault_state_pda, false),
    ];

    let unpause_ix_data = boring_vault_svm::client::args::Unpause { vault_id }.data();

    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts,
        data: unpause_ix_data,
    };

    Ok(instruction)
}

pub fn create_transfer_authority_instruction(
    vault_id: u64,
    signer: &Pubkey,
    pending_authority: &Pubkey,
) -> Result<Instruction> {
    let vault_state_pda = get_vault_state_pda(vault_id);

    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(vault_state_pda, false),
    ];

    let transfer_authority_ix_data = boring_vault_svm::client::args::TransferAuthority {
        vault_id,
        pending_authority: *pending_authority,
    }
    .data();

    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts,
        data: transfer_authority_ix_data,
    };

    Ok(instruction)
}

pub fn create_accept_authority_instruction(vault_id: u64, signer: &Pubkey) -> Result<Instruction> {
    let vault_state_pda = get_vault_state_pda(vault_id);

    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(vault_state_pda, false),
    ];

    let accept_authority_ix_data =
        boring_vault_svm::client::args::AcceptAuthority { vault_id }.data();

    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts,
        data: accept_authority_ix_data,
    };

    Ok(instruction)
}

pub fn create_close_cpi_digest_instruction(
    vault_id: u64,
    signer: &Pubkey,
    digest: [u8; 32],
) -> Result<Instruction> {
    let vault_state_pda = get_vault_state_pda(vault_id);
    let cpi_digest_pda = get_cpi_digest_pda(vault_id, digest);

    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(vault_state_pda, false),
        AccountMeta::new(cpi_digest_pda, false),
    ];

    let close_cpi_digest_ix_data =
        boring_vault_svm::client::args::CloseCpiDigest { vault_id, digest }.data();

    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts,
        data: close_cpi_digest_ix_data,
    };

    Ok(instruction)
}

pub fn create_update_exchange_rate_provider_instruction(
    vault_id: u64,
    signer: &Pubkey,
    new_provider: &Pubkey,
) -> Result<Instruction> {
    let vault_state_pda = get_vault_state_pda(vault_id);

    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(vault_state_pda, false),
    ];

    let update_exchange_rate_provider_ix_data =
        boring_vault_svm::client::args::UpdateExchangeRateProvider {
            vault_id,
            new_provider: *new_provider,
        }
        .data();

    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts,
        data: update_exchange_rate_provider_ix_data,
    };

    Ok(instruction)
}

pub fn create_set_withdraw_authority_instruction(
    vault_id: u64,
    signer: &Pubkey,
    new_authority: &Pubkey,
) -> Result<Instruction> {
    let vault_state_pda = get_vault_state_pda(vault_id);

    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(vault_state_pda, false),
    ];

    let set_withdraw_authority_ix_data = boring_vault_svm::client::args::SetWithdrawAuthority {
        vault_id,
        new_authority: *new_authority,
    }
    .data();

    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts,
        data: set_withdraw_authority_ix_data,
    };

    Ok(instruction)
}

pub fn create_set_payout_instruction(
    vault_id: u64,
    signer: &Pubkey,
    new_payout: &Pubkey,
) -> Result<Instruction> {
    let vault_state_pda = get_vault_state_pda(vault_id);

    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(vault_state_pda, false),
    ];

    let set_payout_ix_data = boring_vault_svm::client::args::SetPayout {
        vault_id,
        new_payout: *new_payout,
    }
    .data();

    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts,
        data: set_payout_ix_data,
    };

    Ok(instruction)
}

pub fn create_configure_exchange_rate_update_bounds_instruction(
    vault_id: u64,
    signer: &Pubkey,
    upper_bound: u16,
    lower_bound: u16,
    minimum_update_delay: u32,
) -> Result<Instruction> {
    let vault_state_pda = get_vault_state_pda(vault_id);

    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(vault_state_pda, false),
    ];

    let args = boring_vault_svm::types::ConfigureExchangeRateUpdateBoundsArgs {
        upper_bound,
        lower_bound,
        minimum_update_delay,
    };

    let configure_exchange_rate_update_bounds_ix_data =
        boring_vault_svm::client::args::ConfigureExchangeRateUpdateBounds { vault_id, args }.data();

    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts,
        data: configure_exchange_rate_update_bounds_ix_data,
    };

    Ok(instruction)
}

pub fn create_set_fees_instruction(
    vault_id: u64,
    signer: &Pubkey,
    platform_fee_bps: u16,
    performance_fee_bps: u16,
) -> Result<Instruction> {
    let vault_state_pda = get_vault_state_pda(vault_id);

    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(vault_state_pda, false),
    ];

    let set_fees_ix_data = boring_vault_svm::client::args::SetFees {
        vault_id,
        platform_fee_bps,
        performance_fee_bps,
    }
    .data();

    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts,
        data: set_fees_ix_data,
    };

    Ok(instruction)
}

pub fn create_set_strategist_instruction(
    vault_id: u64,
    signer: &Pubkey,
    new_strategist: &Pubkey,
) -> Result<Instruction> {
    let vault_state_pda = get_vault_state_pda(vault_id);

    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(vault_state_pda, false),
    ];

    let set_strategist_ix_data = boring_vault_svm::client::args::SetStrategist {
        vault_id,
        new_strategist: *new_strategist,
    }
    .data();

    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts,
        data: set_strategist_ix_data,
    };

    Ok(instruction)
}

pub fn create_update_exchange_rate_instruction(
    vault_id: u64,
    signer: &Pubkey,
    new_exchange_rate: u64,
) -> Result<Instruction> {
    let vault_state_pda = get_vault_state_pda(vault_id);

    let share_mint = get_vault_share_mint(vault_state_pda);

    let accounts = boring_vault_svm::client::accounts::UpdateExchangeRate {
        signer: *signer,
        boring_vault_state: vault_state_pda,
        share_mint,
    };

    let update_exchange_rate_ix_data = boring_vault_svm::client::args::UpdateExchangeRate {
        vault_id,
        new_exchange_rate,
    }
    .data();

    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts: accounts.to_account_metas(None),
        data: update_exchange_rate_ix_data,
    };

    Ok(instruction)
}

pub fn create_claim_fees_in_base_instruction(
    client: &RpcClient,
    vault_id: u64,
    sub_account: u8,
    signer: &Pubkey,
) -> Result<Vec<Instruction>> {
    let vault_state_pda = get_vault_state_pda(vault_id);
    let vault_pda = get_vault_pda(vault_id, sub_account);
    let vault_state_account = client.get_account(&vault_state_pda)?;
    let vault_state = boring_vault_svm::accounts::BoringVault::try_deserialize(
        &mut &vault_state_account.data[..],
    )?;

    let base_asset = vault_state.teller.base_asset;
    let payout_address = vault_state.teller.payout_address;
    let base_mint_account = client.get_account(&base_asset)?;
    let token_program_id = base_mint_account.owner;

    let payout_ata: Pubkey = get_associated_token_address_with_program_id(
        &payout_address,
        &base_asset,
        &token_program_id,
    );

    let mut instructions = vec![];
    if let Err(_) = client.get_account(&payout_ata) {
        instructions.push(create_associated_token_account(
            signer,
            &payout_address,
            &base_asset,
            &token_program_id,
        ));
    } 

    let vault_ata =
        get_associated_token_address_with_program_id(&vault_pda, &base_asset, &token_program_id);

    if let Err(_) = client.get_account(&vault_ata) {
        instructions.push(create_associated_token_account(
            signer,
            &vault_pda,
            &base_asset,
            &token_program_id,
        ));
    }

    let accounts = boring_vault_svm::client::accounts::ClaimFeesInBase {
        signer: *signer,
        base_mint: base_asset,
        boring_vault_state: vault_state_pda,
        boring_vault: vault_pda,
        payout_ata,
        vault_ata,
        token_program: spl_token::ID,
        token_program_2022: TOKEN_2022_PROGRAM_ID,
        system_program: system_program::ID,
    };

    let claim_fees_in_base_ix_data = boring_vault_svm::client::args::ClaimFeesInBase {
        vault_id,
        sub_account,
    }
    .data();

    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts: accounts.to_account_metas(None),
        data: claim_fees_in_base_ix_data,
    };

    instructions.push(instruction);

    Ok(instructions)
}

pub fn create_deposit_instruction(
    client: &RpcClient,
    vault_id: u64,
    signer: &Pubkey,
    deposit_mint: &Pubkey,
    deposit_amount: u64,
    min_mint_amount: u64,
) -> Result<Vec<Instruction>> {
    let vault_state_pda = get_vault_state_pda(vault_id);
    let vault_state_account = client.get_account(&vault_state_pda)?;
    let vault_state = boring_vault_svm::accounts::BoringVault::try_deserialize(
        &mut &vault_state_account.data[..],
    )?;

    let vault_pda = get_vault_pda(vault_id, vault_state.config.deposit_sub_account);
    let share_mint = get_vault_share_mint(vault_state_pda);

    let asset_data_pda = get_asset_data_pda(vault_state_pda, *deposit_mint);
    let asset_data_account = client.get_account(&asset_data_pda)?;
    let asset_data =
        boring_vault_svm::accounts::AssetData::try_deserialize(&mut &asset_data_account.data[..])?;
    let price_feed = asset_data.price_feed;

    let deposit_mint_account = client.get_account(deposit_mint)?;
    let token_program_id = deposit_mint_account.owner;

    let mut instructions = vec![];

    let user_ata =
        get_associated_token_address_with_program_id(signer, deposit_mint, &token_program_id);

    if let Err(_) = client.get_account(&user_ata) {
        instructions.push(create_associated_token_account(
            signer,
            &signer,
            &deposit_mint,
            &token_program_id,
            ));
        }

    let vault_ata =
        get_associated_token_address_with_program_id(&vault_pda, deposit_mint, &token_program_id);

    if let Err(_) = client.get_account(&vault_ata) {
        instructions.push(create_associated_token_account(
            signer,
            &vault_pda,
            &deposit_mint,
            &token_program_id,
        ));
    }

    let user_share_ata =
        get_associated_token_address_with_program_id(signer, &share_mint, &TOKEN_2022_PROGRAM_ID);

    if let Err(_) = client.get_account(&user_share_ata) {
        instructions.push(create_associated_token_account(
            signer,
            &signer,
            &share_mint,
            &TOKEN_2022_PROGRAM_ID,
        ));
    }

    let accounts = boring_vault_svm::client::accounts::Deposit {
        signer: *signer,
        boring_vault_state: vault_state_pda,
        boring_vault: vault_pda,
        deposit_mint: *deposit_mint,
        asset_data: asset_data_pda,
        user_ata,
        vault_ata,
        token_program: spl_token::ID,
        token_program_2022: TOKEN_2022_PROGRAM_ID,
        system_program: system_program::ID,
        associated_token_program: ASSOCIATED_TOKEN_PROGRAM_ID,
        share_mint,
        user_shares: user_share_ata,
        price_feed,
    };

    let args = boring_vault_svm::types::DepositArgs {
        vault_id,
        deposit_amount,
        min_mint_amount,
    };

    let deposit_ix_data = boring_vault_svm::client::args::Deposit { args }.data();

    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts: accounts.to_account_metas(None),
        data: deposit_ix_data,
    };

    instructions.push(instruction);

    Ok(instructions)
}

pub fn create_withdraw_instruction(
    client: &RpcClient,
    vault_id: u64,
    signer: &Pubkey,
    withdraw_mint: &Pubkey,
    share_amount: u64,
    min_assets_amount: u64,
) -> Result<Instruction> {
    let vault_state_pda = get_vault_state_pda(vault_id);
    let vault_state_account = client.get_account(&vault_state_pda)?;
    let vault_state = boring_vault_svm::accounts::BoringVault::try_deserialize(
        &mut &vault_state_account.data[..],
    )?;

    let withdraw_sub_account = vault_state.config.withdraw_sub_account;
    let vault_pda = get_vault_pda(vault_id, withdraw_sub_account);
    let share_mint = get_vault_share_mint(vault_state_pda);

    let asset_data_pda = get_asset_data_pda(vault_state_pda, *withdraw_mint);
    let asset_data_account = client.get_account(&asset_data_pda)?;
    let asset_data =
        boring_vault_svm::accounts::AssetData::try_deserialize(&mut &asset_data_account.data[..])?;
    let price_feed = asset_data.price_feed;

    let withdraw_mint_account = client.get_account(withdraw_mint)?;
    let token_program_id = withdraw_mint_account.owner;

    let user_ata =
        get_associated_token_address_with_program_id(signer, withdraw_mint, &token_program_id);

    let vault_ata =
        get_associated_token_address_with_program_id(&vault_pda, withdraw_mint, &token_program_id);

    let user_share_ata =
        get_associated_token_address_with_program_id(signer, &share_mint, &TOKEN_2022_PROGRAM_ID);

    let accounts = boring_vault_svm::client::accounts::Withdraw {
        signer: *signer,
        boring_vault_state: vault_state_pda,
        boring_vault: vault_pda,
        withdraw_mint: *withdraw_mint,
        asset_data: asset_data_pda,
        user_ata,
        vault_ata,
        token_program: spl_token::ID,
        token_program_2022: TOKEN_2022_PROGRAM_ID,
        share_mint,
        user_shares: user_share_ata,
        price_feed,
    };

    let args = boring_vault_svm::types::WithdrawArgs {
        vault_id,
        share_amount,
        min_assets_amount,
    };

    let withdraw_ix_data = boring_vault_svm::client::args::Withdraw { args }.data();

    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts: accounts.to_account_metas(None),
        data: withdraw_ix_data,
    };

    Ok(instruction)
}

// STRATEGIST INSTRUCTIONS

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
    client: &RpcClient,
    signer: &Pubkey,
    vault_id: u64,
    user_pubkey: &Pubkey,
    deposit_amount: u64,
    min_mint_amount: u64,
) -> Result<Instruction> {
    let vault_state_pda = get_vault_state_pda(vault_id);
    let vault_state_account = client.get_account(&vault_state_pda)?;
    let vault_state_data = boring_vault_svm::accounts::BoringVault::try_deserialize(
        &mut &vault_state_account.data[..],
    )?;
    let native_mint = Pubkey::new_from_array([0; 32]);
    let asset_data_pda = get_asset_data_pda(vault_state_pda, native_mint);
    let vault_pda = get_vault_pda(vault_id, vault_state_data.config.deposit_sub_account);
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
