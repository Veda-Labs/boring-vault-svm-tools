use anchor_lang::pubkey;
use eyre::Result;
use solana_client::rpc_client::RpcClient;
use solana_instruction::Instruction;
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use spl_token::ID as TOKEN_PROGRAM_ID;

use crate::{
    manage_instructions::SolendDepositReserveLiquidityAndObligationCollateral, utils::get_vault_pda,
};

use super::{create_manage_instruction, init_associated_token_account_if_needed};

// TODO this tx is too big if u send all at once
pub fn create_deposit_solend_instructions(
    client: &RpcClient,
    signer: &Keypair,
    authority: Option<&Keypair>,
    vault_id: u64,
    sub_account: u8,
    deposit_mint: &Pubkey,
    reserve_collateral_mint: &Pubkey,
    lending_market: &Pubkey,
    reserve: &Pubkey,
    reserve_liquidity_supply_spl_token_account: &Pubkey,
    lending_market_authority: &Pubkey,
    destination_deposit_reserve_collateral_supply_spl_token_account: &Pubkey,
    pyth_price_oracle: &Pubkey,
    switchboard_price_oracle: &Pubkey,
    amount: u64,
) -> Result<Vec<Instruction>> {
    let vault_pda = get_vault_pda(vault_id, sub_account);

    let mut instructions = vec![];

    // Init ATA if needed
    if let Some(ix) = init_associated_token_account_if_needed(
        client,
        &signer.pubkey(),
        vault_id,
        sub_account,
        reserve_collateral_mint,
        &TOKEN_PROGRAM_ID,
    )? {
        instructions.push(ix);
    }
    // Create account with seed
    // let eix_0 = CreateAccountWithSeed::new(
    //     vault_id,
    //     sub_account,
    //     "4UpD2fh7xH3VP9QQaXtsS1YY3bxzWhtf".to_string(),
    //     9938880,
    //     1300,
    //     pubkey!("So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo"),
    // );
    // let ixs = create_manage_instruction(client, signer, authority, eix_0)?;
    // instructions.extend(ixs);

    // Init obligation.
    let obligation = Pubkey::create_with_seed(
        &vault_pda,
        "4UpD2fh7xH3VP9QQaXtsS1YY3bxzWhtf",
        &pubkey!("So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo"),
    )?;
    // let eix_1 = SolendInitObligation::new(vault_id, sub_account, obligation, *lending_market);
    // let ixs = create_manage_instruction(client, signer, authority, eix_1)?;
    // instructions.extend(ixs);

    // Deposit
    let eix_2 = SolendDepositReserveLiquidityAndObligationCollateral::new(
        vault_id,
        sub_account,
        *deposit_mint,
        *reserve_collateral_mint,
        *reserve,
        *reserve_liquidity_supply_spl_token_account,
        *lending_market,
        *lending_market_authority,
        *destination_deposit_reserve_collateral_supply_spl_token_account,
        obligation,
        *pyth_price_oracle,
        *switchboard_price_oracle,
        amount,
    );
    let ixs = create_manage_instruction(client, signer, authority, eix_2)?;
    instructions.extend(ixs);

    Ok(instructions)
}
