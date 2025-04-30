use crate::impl_external_instruction_common;
use crate::manage_instructions::ExternalInstruction;
use crate::utils::bindings::boring_vault_svm::types::{Operator, Operators};
use crate::utils::pdas;

use solana_instruction::account_meta::AccountMeta;
use solana_pubkey::{pubkey, Pubkey};
use spl_associated_token_account::get_associated_token_address_with_program_id;
use spl_token::ID as TOKEN_PROGRAM_ID;

const SOLEND_PROGRAM_ID: Pubkey = pubkey!("So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo");

pub struct SolendInitObligation {
    vault_id: u64,
    sub_account: u8,
    obligation: Pubkey,
    lending_market: Pubkey,
}

impl SolendInitObligation {
    pub fn new(vault_id: u64, sub_account: u8, obligation: Pubkey, lending_market: Pubkey) -> Self {
        Self {
            vault_id,
            sub_account,
            obligation,
            lending_market,
        }
    }
}

impl ExternalInstruction for SolendInitObligation {
    impl_external_instruction_common!();

    fn ix_program_id(&self) -> Pubkey {
        SOLEND_PROGRAM_ID
    }

    fn ix_data(&self) -> Vec<u8> {
        hex::decode("06").expect("Failed to decode hex discriminator")
    }

    fn ix_remaining_accounts(&self) -> Vec<AccountMeta> {
        let owner = pdas::get_vault_pda(self.vault_id, self.sub_account);
        vec![
            AccountMeta::new(self.obligation, false), // obligation
            AccountMeta::new_readonly(self.lending_market, false), // lending market
            AccountMeta::new(owner, false),           // owner
            AccountMeta::new_readonly(solana_program::sysvar::rent::ID, false), // rent
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false), // token program
        ]
    }

    fn ix_operators(&self) -> Operators {
        let operators = vec![
            Operator::IngestInstruction(0, 1),
            Operator::IngestAccount(1),
            Operator::IngestInstructionDataSize,
        ];

        Operators { operators }
    }
}

pub struct SolendDepositReserveLiquidityAndObligationCollateral {
    vault_id: u64,
    sub_account: u8,
    deposit_mint: Pubkey,
    reserve_collateral_mint: Pubkey,
    reserve: Pubkey,
    reserve_liquidity_supply_spl_token_account: Pubkey,
    lending_market: Pubkey,
    lending_market_authority: Pubkey,
    destination_deposit_reserve_collateral_supply_spl_token_account: Pubkey,
    obligation: Pubkey,
    pyth_price_oracle: Pubkey,
    switchboard_price_oracle: Pubkey,
    amount: u64,
}

impl SolendDepositReserveLiquidityAndObligationCollateral {
    pub fn new(
        vault_id: u64,
        sub_account: u8,
        deposit_mint: Pubkey,
        reserve_collateral_mint: Pubkey,
        reserve: Pubkey,
        reserve_liquidity_supply_spl_token_account: Pubkey,
        lending_market: Pubkey,
        lending_market_authority: Pubkey,
        destination_deposit_reserve_collateral_supply_spl_token_account: Pubkey,
        obligation: Pubkey,
        pyth_price_oracle: Pubkey,
        switchboard_price_oracle: Pubkey,
        amount: u64,
    ) -> Self {
        Self {
            vault_id,
            sub_account,
            deposit_mint,
            reserve_collateral_mint,
            reserve,
            reserve_liquidity_supply_spl_token_account,
            lending_market,
            lending_market_authority,
            destination_deposit_reserve_collateral_supply_spl_token_account,
            obligation,
            pyth_price_oracle,
            switchboard_price_oracle,
            amount,
        }
    }
}

impl ExternalInstruction for SolendDepositReserveLiquidityAndObligationCollateral {
    impl_external_instruction_common!();

    fn ix_program_id(&self) -> Pubkey {
        pubkey!("So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo")
    }

    fn ix_data(&self) -> Vec<u8> {
        // 0e40420f0000000000
        let mut ix_data = hex::decode("0e").expect("Failed to decode hex discriminator");
        ix_data.extend_from_slice(&self.amount.to_le_bytes());
        ix_data
    }

    fn ix_remaining_accounts(&self) -> Vec<AccountMeta> {
        let owner = pdas::get_vault_pda(self.vault_id, self.sub_account);
        let vault_deposit_ata = get_associated_token_address_with_program_id(
            &owner,
            &self.deposit_mint,
            &TOKEN_PROGRAM_ID,
        );
        let vault_share_ata = get_associated_token_address_with_program_id(
            &owner,
            &self.reserve_collateral_mint,
            &TOKEN_PROGRAM_ID,
        );
        vec![
            AccountMeta::new(vault_deposit_ata, false), // where deposit comes from
            AccountMeta::new(vault_share_ata, false),   // where shares go
            AccountMeta::new(self.reserve, false),      // reserve
            AccountMeta::new(self.reserve_liquidity_supply_spl_token_account, false), // reserve_liquidity_supply_spl_token_account
            AccountMeta::new(self.reserve_collateral_mint, false), // reserve_collateral_mint
            AccountMeta::new(self.lending_market, false),          // lending market
            AccountMeta::new(self.lending_market_authority, false), // lending market authority
            AccountMeta::new(
                self.destination_deposit_reserve_collateral_supply_spl_token_account,
                false,
            ), // destination_deposit_reserve_collateral_supply_spl_token_account
            AccountMeta::new(self.obligation, false),              // obligation
            AccountMeta::new(owner, false),                        // owner
            AccountMeta::new_readonly(self.pyth_price_oracle, false), // pyth price oracle
            AccountMeta::new_readonly(self.switchboard_price_oracle, false), // switchboard price oracle
            AccountMeta::new(owner, false), // user transfer authority
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false), // token program
        ]
    }

    fn ix_operators(&self) -> Operators {
        let operators = vec![
            Operator::IngestInstruction(0, 1),
            Operator::IngestAccount(1),
            Operator::IngestInstructionDataSize,
        ];

        Operators { operators }
    }
}
