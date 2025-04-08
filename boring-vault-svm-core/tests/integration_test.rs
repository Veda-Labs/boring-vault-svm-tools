use anchor_client::solana_sdk::signature::Keypair;
use anchor_lang::{prelude::Clock, Discriminator, InstructionData};
use litesvm::LiteSVM;
use solana_account::Account;
use solana_address_lookup_table_interface::instruction::create_lookup_table;
use solana_client::rpc_client::RpcClient;
use solana_instruction::account_meta::AccountMeta;
use solana_program::sysvar::rent::ID as RENT_ID;
use solana_program::{program_option::COption, system_program};
use solana_program_pack::Pack;
use solana_pubkey::{pubkey, Pubkey};
use solana_signer::Signer;
use solana_transaction::Transaction;
use spl_associated_token_account_client::address::get_associated_token_address;
use spl_token::{
    state::{Account as TokenAccount, AccountState},
    ID as TOKEN_PROGRAM_ID,
};
use spl_token_2022::ID as TOKEN_2022_PROGRAM_ID;
use std::str::FromStr;

anchor_lang::declare_program!(boring_vault_svm);

// Stake Pool Program
const STAKE_POOL_PROGRAM_ID: &str = "SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy";
const JITO_SOL_STAKE_POOL: &str = "Jito4APyf642JPZPx3hGc6WWJ8zPKtRbRs4P815Awbb";
const JITO_SOL_STAKE_POOL_WITHDRAW_AUTH: &str = "6iQKfEyhr3bZMotVkW6beNZz5CPAkiwvgV2CTje9pVSS";
const JITO_SOL_STAKE_POOL_RESERVE: &str = "BgKUXdS29YcHCFrPm5M8oLHiTzZaMDjsebggjoaQ6KFL";
const JITO_SOL_STAKE_POOL_FEE: &str = "feeeFLLsam6xZJFc6UQFrHqkvVt4jfmVvi2BRLkUZ4i";
const JITOSOL_SOL_ORACLE: &str = "4Z1SLH9g4ikNBV8uP2ZctEouqjYmVqB2Tz5SZxKYBN7z";
const JITOSOL: &str = "J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn";
const ADDRESS_LOOKUP_TABLE_PROGRAM_ID: &str = "AddressLookupTab1e1111111111111111111111111";
const KAMINO_LEND_PROGRAM_ID: &str = "KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD";
const KAMINO_LEND_JITO_SOL_OBLIGATION: &str = "95XivWGu4By7b7B6upK5ThXrYSsKKtNGrcpcgucTStNU";
const KAMINO_LEND_JITO_SOL_MARKET: &str = "7u3HeHxYDLhnCoErrtycNokbQYbWGzLs6JSDqGAv5PfF";
const WSOL: &str = "So11111111111111111111111111111111111111112";
const SOLEND_PROGRAM_ID: &str = "So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo";
const SOLEND_MAIN_POOL_LENDING_MARKET: &str = "4UpD2fh7xH3VP9QQaXtsS1YY3bxzWhtfpks7FatyKvdY";
const SOLEND_MAIN_POOL_JITOSOL: &str = "6mFgUsvXQTEYrYgowc9pVzYi49XEJA5uHA9gVDURc2pM";
const SOLEND_SOURCE_LIQUIDITY_TOKEN_ACCOUNT: &str = "BF79wh4Zqgq74kF1DE97VuciseZnyrbC9TbQ9xmDViR1";
const SOLEND_RESERVE_ACCOUNT: &str = "BRsz1xVQMuVLbc4YjLP1FXhEx1LxSYig2nLqRgJEzR9r";
const SOLEND_RESERVE_LIQUIDITY_SUPPLY_SPL_TOKEN_ACCOUNT: &str =
    "2Khz77qDAL4yY1wG6mTLhLnKiN7sDjQCtrFDEEUFPpiB";
const SOLEND_MAIN_POOL_LENDING_AUTHORITY: &str = "DdZR6zRFiUt4S5mg7AV1uKB2z1f1WzcNYCaTEEWPAuby";
const SOLEND_DESTINATION_DEPOSIT_RESERVE_COLLATERAL_SUPPLY_SPL_TOKEN_ACCOUNT: &str =
    "3GynM9cRtZsZ2s1SyoAuSgTDjx8ANcVZJXZayuWZbMpd";
const SOLEND_PYTH_PRICE_ORACLE_SOL: &str = "7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE";

// Function to get all accounts for cloning
fn get_accounts_to_clone() -> Vec<String> {
    vec![
        JITO_SOL_STAKE_POOL.to_string(),
        JITO_SOL_STAKE_POOL_WITHDRAW_AUTH.to_string(),
        JITO_SOL_STAKE_POOL_RESERVE.to_string(),
        JITO_SOL_STAKE_POOL_FEE.to_string(),
        JITOSOL_SOL_ORACLE.to_string(),
        JITOSOL.to_string(),
        KAMINO_LEND_JITO_SOL_OBLIGATION.to_string(),
        KAMINO_LEND_JITO_SOL_MARKET.to_string(),
        WSOL.to_string(),
        SOLEND_MAIN_POOL_LENDING_MARKET.to_string(),
        SOLEND_MAIN_POOL_JITOSOL.to_string(),
        SOLEND_SOURCE_LIQUIDITY_TOKEN_ACCOUNT.to_string(),
        SOLEND_RESERVE_ACCOUNT.to_string(),
        SOLEND_RESERVE_LIQUIDITY_SUPPLY_SPL_TOKEN_ACCOUNT.to_string(),
        SOLEND_MAIN_POOL_LENDING_AUTHORITY.to_string(),
        SOLEND_DESTINATION_DEPOSIT_RESERVE_COLLATERAL_SUPPLY_SPL_TOKEN_ACCOUNT.to_string(),
        SOLEND_PYTH_PRICE_ORACLE_SOL.to_string(),
    ]
}

pub fn run_test() {
    let mut svm = LiteSVM::new();
    println!("Program ID: {}", boring_vault_svm::ID);

    let deployer = create_test_keypair(&mut svm, 10_000_000_000);
    let authority = create_test_keypair(&mut svm, 10_000_000_000);

    // Add programs.
    svm.add_program_from_file(boring_vault_svm::ID, "./program_bytes/boring_vault_svm.so")
        .unwrap();
    svm.add_program_from_file(
        Pubkey::from_str(STAKE_POOL_PROGRAM_ID).unwrap(),
        "./program_bytes/sol_stake_pool.so",
    )
    .unwrap();
    svm.add_program_from_file(
        Pubkey::from_str(KAMINO_LEND_PROGRAM_ID).unwrap(),
        "./program_bytes/kamino_lend.so",
    )
    .unwrap();
    svm.add_program_from_file(
        Pubkey::from_str(SOLEND_PROGRAM_ID).unwrap(),
        "./program_bytes/solend.so",
    )
    .unwrap();

    // Clone accounts from mainnet
    clone_accounts_from_mainnet(&mut svm, get_accounts_to_clone());

    let vault_0 = get_vault_pda(0, 0);
    // Give vault pda 0 SOL.
    svm.airdrop(&vault_0, 10_000_000_000).unwrap();

    // Deal tokens.
    create_ata_and_mint(
        &mut svm,
        authority.pubkey(),
        Pubkey::from_str(JITOSOL).unwrap(),
        1_000_000_000,
    );
    create_ata_and_mint(
        &mut svm,
        vault_0,
        Pubkey::from_str(JITOSOL).unwrap(),
        1_000_000_000,
    );

    // Initialize boring vault program.
    create_initialize_tx(&mut svm, &deployer.pubkey(), &deployer);

    // Deploy boring vault.
    create_deploy_tx(&mut svm, &authority.pubkey(), &deployer, 0);

    // Create LUT for vault_0.
    let lut_vault_0 = create_lut(&mut svm, &deployer, &vault_0);

    // Initialize CPI Digest.
    let operators = boring_vault_svm::types::Operators {
        operators: vec![boring_vault_svm::types::Operator::Noop],
    };

    // Create the instruction data for init_user_metadata
    let discriminator = hex::decode("75a9b045c5170fa2").unwrap();
    let mut init_obligation_ix_data = discriminator;
    init_obligation_ix_data.extend_from_slice(&lut_vault_0.to_bytes());
    let cpi_digest_pda = create_initialize_cpi_digest_tx(
        &mut svm,
        &authority,
        0,
        Pubkey::from_str(KAMINO_LEND_PROGRAM_ID).unwrap(),
        init_obligation_ix_data.clone(),
        operators,
        33,
    );
    // Call manage
    create_manage_tx(
        &mut svm,
        &authority,
        vault_0,
        cpi_digest_pda,
        0,
        0,
        Pubkey::from_str(KAMINO_LEND_PROGRAM_ID).unwrap(),
        init_obligation_ix_data,
    );
}

fn create_lut(svm: &mut LiteSVM, signer: &Keypair, authority: &Pubkey) -> Pubkey {
    let clock = svm.get_sysvar::<Clock>();
    let recent_slot = clock.slot;
    // println!("Slot: {}", clock.slot);
    // svm.warp_to_slot(1);
    // let clock = svm.get_sysvar::<Clock>(); // Get fresh clock
    // println!("Slot: {}", clock.slot);

    // Create the lookup table instruction
    let (lookup_table_ix, lookup_table_address) = create_lookup_table(
        *authority,      // authority
        signer.pubkey(), // payer
        recent_slot,
    );

    // Create the transaction
    let transaction = Transaction::new_signed_with_payer(
        &[lookup_table_ix],
        Some(&signer.pubkey()),
        &[signer],
        svm.latest_blockhash(),
    );

    svm.send_transaction(transaction).unwrap();
    println!("Lookup table created at: {}", lookup_table_address);

    lookup_table_address
}

fn create_ata_and_mint(svm: &mut LiteSVM, owner: Pubkey, mint: Pubkey, amount: u64) -> Pubkey {
    let ata = get_associated_token_address(&owner, &mint);
    let token_acc = TokenAccount {
        mint,
        owner,
        amount,
        delegate: COption::None,
        state: AccountState::Initialized,
        is_native: COption::None,
        delegated_amount: 0,
        close_authority: COption::None,
    };
    let mut token_acc_bytes = [0u8; TokenAccount::LEN];
    TokenAccount::pack(token_acc, &mut token_acc_bytes).unwrap();
    svm.set_account(
        ata,
        Account {
            lamports: 1_000_000_000,
            data: token_acc_bytes.to_vec(),
            owner: TOKEN_PROGRAM_ID,
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();

    let raw_account = svm.get_account(&ata).unwrap();
    assert_eq!(
        TokenAccount::unpack(&raw_account.data).unwrap().amount,
        amount
    );

    ata
}

fn clone_accounts_from_mainnet(svm: &mut LiteSVM, accounts: Vec<String>) {
    let client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());

    for account in accounts {
        let pubkey = Pubkey::from_str(&account).unwrap();
        match client.get_account(&pubkey) {
            Ok(account) => {
                svm.set_account(pubkey, account).unwrap();
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}

fn create_initialize_tx(svm: &mut LiteSVM, authority: &Pubkey, signer: &Keypair) {
    let accounts = vec![
        AccountMeta::new(signer.pubkey(), true),
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

    let recent_blockhash = svm.latest_blockhash();

    // Create the transaction
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signer.pubkey()),
        &[signer],
        recent_blockhash,
    );
    // let transaction = Transaction::new(&[&signer], message, recent_blockhash);
    let tx_res = svm.send_transaction(transaction).unwrap();

    println!("Boring Vault Initialized! {:?}", tx_res.pretty_logs());
}

fn create_deploy_tx(svm: &mut LiteSVM, authority: &Pubkey, signer: &Keypair, vault_id: u64) {
    let vault_state_pda = get_vault_state_pda(vault_id);
    let accounts = vec![
        AccountMeta::new(signer.pubkey(), true),
        AccountMeta::new(get_program_config_pda(), false),
        AccountMeta::new(vault_state_pda, false),
        AccountMeta::new(get_vault_share_mint(vault_state_pda), false),
        AccountMeta::new_readonly(Pubkey::from_str(JITOSOL).unwrap(), false),
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new_readonly(TOKEN_2022_PROGRAM_ID, false),
    ];

    let args = boring_vault_svm::types::DeployArgs {
        authority: *authority,
        name: "Test Boring Vault".to_string(),
        symbol: "TBV".to_string(),
        exchange_rate_provider: *authority,
        exchange_rate: 1_000_000_000,
        payout_address: *authority,
        allowed_exchange_rate_change_upper_bound: 10_100,
        allowed_exchange_rate_change_lower_bound: 9_900,
        minimum_update_delay_in_seconds: 3_600,
        platform_fee_bps: 0,
        performance_fee_bps: 0,
        withdraw_authority: Pubkey::default(),
        strategist: *authority,
    };
    let deploy_ix_data = boring_vault_svm::client::args::Deploy { args }.data();

    // Create the instruction
    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts,
        data: deploy_ix_data,
    };

    let recent_blockhash = svm.latest_blockhash();

    // Create the transaction
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signer.pubkey()),
        &[signer],
        recent_blockhash,
    );
    // let transaction = Transaction::new(&[&signer], message, recent_blockhash);
    let tx_res = svm.send_transaction(transaction).unwrap();

    println!("Boring Vault Deployed! {:?}", tx_res.pretty_logs());
}

fn create_manage_tx(
    svm: &mut LiteSVM,
    signer: &Keypair,
    vault_account: Pubkey,
    cpi_digest_pda: Pubkey,
    vault_id: u64,
    sub_account: u8,
    ix_program_id: Pubkey,
    ix_data: Vec<u8>,
) {
    let vault_state_pda = get_vault_state_pda(vault_id);
    // Derive vault_account metadata pda.
    let (user_metadata_pda, _) =
        Pubkey::find_program_address(&[b"user_meta", vault_account.as_ref()], &ix_program_id);
    let accounts = vec![
        AccountMeta::new(signer.pubkey(), true),
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

    let recent_blockhash = svm.latest_blockhash();

    // Create the transaction
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signer.pubkey()),
        &[signer],
        recent_blockhash,
    );
    // let transaction = Transaction::new(&[&signer], message, recent_blockhash);
    let tx_res = svm.send_transaction(transaction).unwrap();

    println!("Boring Vault Managed! {:?}", tx_res.pretty_logs());
}

fn create_initialize_cpi_digest_tx(
    svm: &mut LiteSVM,
    signer: &Keypair,
    vault_id: u64,
    ix_program_id: Pubkey,
    ix_data: Vec<u8>,
    operators: boring_vault_svm::types::Operators,
    expected_size: u16,
) -> Pubkey {
    let (cpi_digest_pda, digest) = get_cpi_digest(
        svm,
        signer,
        vault_id,
        ix_program_id,
        ix_data,
        operators.clone(),
        expected_size,
    );

    println!("digest pda: {}", cpi_digest_pda);

    let vault_state_pda = get_vault_state_pda(vault_id);
    let accounts = vec![
        AccountMeta::new(signer.pubkey(), true),
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new(vault_state_pda, false),
        AccountMeta::new(cpi_digest_pda, false),
    ];

    let args = boring_vault_svm::types::CpiDigestArgs {
        vault_id,
        cpi_digest: digest,
        operators,
        expected_size,
    };

    let initialize_cpi_digest_ix_data =
        boring_vault_svm::client::args::InitializeCpiDigest { args }.data();

    // Create the instruction
    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts,
        data: initialize_cpi_digest_ix_data,
    };

    let recent_blockhash = svm.latest_blockhash();

    // Create the transaction
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signer.pubkey()),
        &[signer],
        recent_blockhash,
    );
    // let transaction = Transaction::new(&[&signer], message, recent_blockhash);
    let tx_res = svm.send_transaction(transaction).unwrap();

    println!("Cpi Digest Initialized! {}", hex::encode(digest));

    cpi_digest_pda
}

fn get_cpi_digest(
    svm: &mut LiteSVM,
    signer: &Keypair,
    vault_id: u64,
    ix_program_id: Pubkey,
    ix_data: Vec<u8>,
    operators: boring_vault_svm::types::Operators,
    expected_size: u16,
) -> (Pubkey, [u8; 32]) {
    let accounts = vec![AccountMeta::new_readonly(system_program::ID, false)];

    let args = boring_vault_svm::types::ViewCpiDigestArgs {
        ix_program_id: ix_program_id,
        ix_data: ix_data,
        operators: operators,
        expected_size: expected_size,
    };

    let view_cpi_digest_ix_data = boring_vault_svm::client::args::ViewCpiDigest { args }.data();

    let instruction = solana_program::instruction::Instruction {
        program_id: boring_vault_svm::ID,
        accounts,
        data: view_cpi_digest_ix_data,
    };

    // Create the transaction
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signer.pubkey()),
        &[signer],
        svm.latest_blockhash(),
    );
    // let transaction = Transaction::new(&[&signer], message, recent_blockhash);
    let tx_res = svm.send_transaction(transaction).unwrap();

    println!("View Cpi Digest called: {:?}", tx_res.return_data.data);

    // Convert the digest to a fixed-size array and validate length
    let digest: [u8; 32] = tx_res
        .return_data
        .data
        .try_into()
        .expect("Digest must be exactly 32 bytes long");

    // Get the PDA for this digest
    let cpi_digest_pda = get_cpi_digest_pda(vault_id, digest);

    (cpi_digest_pda, digest)
}

fn get_cpi_digest_pda(vault_id: u64, digest: [u8; 32]) -> Pubkey {
    let (cpi_digest_pda, _) = Pubkey::find_program_address(
        &[b"cpi-digest", &vault_id.to_le_bytes()[..], &digest],
        &boring_vault_svm::ID,
    );
    cpi_digest_pda
}

fn get_program_config_pda() -> Pubkey {
    let (program_config, _) = Pubkey::find_program_address(&[b"config"], &boring_vault_svm::ID);
    program_config
}

fn get_vault_state_pda(vault_id: u64) -> Pubkey {
    let (boring_vault_state, _) = Pubkey::find_program_address(
        &[b"boring-vault-state", &vault_id.to_le_bytes()[..]],
        &boring_vault_svm::ID,
    );
    boring_vault_state
}

fn get_vault_share_mint(vault_state_pda: Pubkey) -> Pubkey {
    let (share_mint, _) = Pubkey::find_program_address(
        &[b"share-token", vault_state_pda.as_ref()],
        &boring_vault_svm::ID,
    );
    share_mint
}

fn get_vault_pda(vault_id: u64, sub_account: u8) -> Pubkey {
    let (vault_pda, _) = Pubkey::find_program_address(
        &[b"boring-vault", &vault_id.to_le_bytes()[..], &[sub_account]],
        &boring_vault_svm::ID,
    );
    vault_pda
}

fn create_test_keypair(svm: &mut LiteSVM, airdrop_balance: u64) -> Keypair {
    println!("Setting up test user");

    let test_keypair = Keypair::new();

    svm.airdrop(&test_keypair.pubkey(), airdrop_balance)
        .unwrap();

    test_keypair
}
