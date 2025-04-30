use crate::impl_external_instruction_common;
use crate::manage_instructions::ExternalInstruction;
use crate::utils::bindings::boring_vault_svm::types::{Operator, Operators};
use crate::utils::pdas;

use solana_instruction::account_meta::AccountMeta;
use solana_program::system_program;
use solana_pubkey::{pubkey, Pubkey};
use spl_associated_token_account::get_associated_token_address_with_program_id;
use spl_token::ID as TOKEN_PROGRAM_ID;

const JITO_MINT: Pubkey = pubkey!("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn");
const JITO_STAKE_POOL: Pubkey = pubkey!("Jito4APyf642JPZPx3hGc6WWJ8zPKtRbRs4P815Awbb");
const JITO_STAKE_POOL_WITHDRAW_AUTHORITY: Pubkey =
    pubkey!("6iQKfEyhr3bZMotVkW6beNZz5CPAkiwvgV2CTje9pVSS");
const JITO_RESERVE_STAKE_ACCOUNT: Pubkey = pubkey!("BgKUXdS29YcHCFrPm5M8oLHiTzZaMDjsebggjoaQ6KFL");
const JITO_FEE_ACCOUNT: Pubkey = pubkey!("feeeFLLsam6xZJFc6UQFrHqkvVt4jfmVvi2BRLkUZ4i");

pub struct MintJitoSol {
    vault_id: u64,
    sub_account: u8,
    amount: u64,
}

impl MintJitoSol {
    pub fn new(vault_id: u64, sub_account: u8, amount: u64) -> Self {
        Self {
            vault_id,
            sub_account,
            amount,
        }
    }
}

impl ExternalInstruction for MintJitoSol {
    impl_external_instruction_common!();

    fn ix_program_id(&self) -> Pubkey {
        pubkey!("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy")
    }

    fn ix_data(&self) -> Vec<u8> {
        let mut ix_data = vec![14]; // 14 is the discriminator for minting JitoSol.
        ix_data.extend(self.amount.to_le_bytes());

        ix_data
    }

    fn ix_remaining_accounts(&self) -> Vec<AccountMeta> {
        let vault_pda = pdas::get_vault_pda(self.vault_id, self.sub_account);
        let jito_sol_ata =
            get_associated_token_address_with_program_id(&vault_pda, &JITO_MINT, &TOKEN_PROGRAM_ID);

        vec![
            AccountMeta::new(JITO_STAKE_POOL, false), // Jito (JitoSol) Stake Pool
            AccountMeta::new_readonly(JITO_STAKE_POOL_WITHDRAW_AUTHORITY, false), // stake pool withdraw authority
            AccountMeta::new(JITO_RESERVE_STAKE_ACCOUNT, false), // reserve stake account
            AccountMeta::new(vault_pda, false),                  // depositor
            AccountMeta::new(jito_sol_ata, false),               // user account
            AccountMeta::new(JITO_FEE_ACCOUNT, false),           // fee account
            AccountMeta::new(jito_sol_ata, false),               // referral fee account
            AccountMeta::new(JITO_MINT, false),                  // token mint
            AccountMeta::new_readonly(system_program::ID, false), // system program
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),  // token program
        ]
    }

    fn ix_operators(&self) -> Operators {
        let operators = vec![
            Operator::IngestInstruction(0, 1),
            Operator::IngestAccount(3),
            Operator::IngestAccount(4),
            Operator::IngestInstructionDataSize,
        ];

        Operators { operators }
    }
}

// call unstake on pool, then need to call deactivate, then finally withdraw after some days
