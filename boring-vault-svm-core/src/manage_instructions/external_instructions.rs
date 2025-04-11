use solana_instruction::account_meta::AccountMeta;
use solana_program::system_program;
use solana_pubkey::{pubkey, Pubkey};

use crate::utils::bindings::boring_vault_svm::types::{Operator, Operators};
use crate::utils::{get_user_metadata_pda, pdas};

pub trait ExternalInstruction {
    fn vault_id(&self) -> u64;
    fn sub_account(&self) -> u8;
    fn ix_program_id(&self) -> Pubkey;
    fn ix_data(&self) -> Vec<u8>;
    fn ix_remaining_accounts(&self) -> Vec<AccountMeta>;
    fn ix_operators(&self) -> Operators;
}

pub struct TransferSol {
    vault_id: u64,
    sub_account: u8,
    to: Pubkey,
    amount: u64,
}

impl TransferSol {
    pub fn new(vault_id: u64, sub_account: u8, to: Pubkey, amount: u64) -> Self {
        Self {
            vault_id,
            sub_account,
            to,
            amount,
        }
    }
}

impl ExternalInstruction for TransferSol {
    fn vault_id(&self) -> u64 {
        self.vault_id
    }
    fn sub_account(&self) -> u8 {
        self.sub_account
    }
    fn ix_program_id(&self) -> Pubkey {
        system_program::ID
    }

    fn ix_data(&self) -> Vec<u8> {
        let mut ix_data = vec![0x02, 0x00, 0x00, 0x00]; // Transfer instruction discriminator
        ix_data.extend_from_slice(&self.amount.to_le_bytes()); // Write amount as little-endian
        ix_data
    }

    fn ix_remaining_accounts(&self) -> Vec<AccountMeta> {
        let from = pdas::get_vault_pda(self.vault_id, self.sub_account);
        let ix_remaining_accounts = vec![
            AccountMeta::new(from, false),
            AccountMeta::new(self.to, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ];
        ix_remaining_accounts
    }

    fn ix_operators(&self) -> Operators {
        let operators = vec![
            Operator::IngestInstruction(0, 4),
            Operator::IngestAccount(1),
            Operator::IngestInstructionDataSize,
        ];

        Operators { operators }
    }
}

pub struct TransferSolBetweenSubAccounts {
    vault_id: u64,
    sub_account: u8,
    to_sub_account: u8,
    amount: u64,
}

impl TransferSolBetweenSubAccounts {
    pub fn new(vault_id: u64, sub_account: u8, to_sub_account: u8, amount: u64) -> Self {
        Self {
            vault_id,
            sub_account,
            to_sub_account,
            amount,
        }
    }
}

impl ExternalInstruction for TransferSolBetweenSubAccounts {
    fn vault_id(&self) -> u64 {
        self.vault_id
    }
    fn sub_account(&self) -> u8 {
        self.sub_account
    }
    fn ix_program_id(&self) -> Pubkey {
        system_program::ID
    }

    fn ix_data(&self) -> Vec<u8> {
        let mut ix_data = vec![0x02, 0x00, 0x00, 0x00]; // Transfer instruction discriminator
        ix_data.extend_from_slice(&self.amount.to_le_bytes()); // Write amount as little-endian
        ix_data
    }

    fn ix_remaining_accounts(&self) -> Vec<AccountMeta> {
        let from = pdas::get_vault_pda(self.vault_id, self.sub_account);
        let to = pdas::get_vault_pda(self.vault_id, self.to_sub_account);
        let ix_remaining_accounts = vec![
            AccountMeta::new(from, false),
            AccountMeta::new(to, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ];
        ix_remaining_accounts
    }

    fn ix_operators(&self) -> Operators {
        let operators = vec![
            Operator::IngestInstruction(0, 4),
            Operator::IngestAccount(1),
            Operator::IngestInstructionDataSize,
        ];

        Operators { operators }
    }
}

// TODO create lut?

pub struct KaminoInitUserMetaData {
    vault_id: u64,
    sub_account: u8,
    lut: Pubkey,
}

impl KaminoInitUserMetaData {
    pub fn new(vault_id: u64, sub_account: u8, lut: Pubkey) -> Self {
        Self {
            vault_id,
            sub_account,
            lut,
        }
    }
}

impl ExternalInstruction for KaminoInitUserMetaData {
    fn vault_id(&self) -> u64 {
        self.vault_id
    }
    fn sub_account(&self) -> u8 {
        self.sub_account
    }
    fn ix_program_id(&self) -> Pubkey {
        pubkey!("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD")
    }

    fn ix_data(&self) -> Vec<u8> {
        let mut ix_data =
            hex::decode("75a9b045c5170fa2").expect("Failed to decode hex discriminator"); // init_user_metadata instruction discriminator
        ix_data.extend_from_slice(&self.lut.to_bytes());
        ix_data
    }

    fn ix_remaining_accounts(&self) -> Vec<AccountMeta> {
        let from = pdas::get_vault_pda(self.vault_id, self.sub_account);
        let ix_remaining_accounts = vec![
            AccountMeta::new(from, false), // owner
            AccountMeta::new(from, false), // fee_payer
            AccountMeta::new(get_user_metadata_pda(&from, &self.ix_program_id()), false), // user_metadata
            AccountMeta::new_readonly(self.ix_program_id(), false), // referrer_user_metadata
            AccountMeta::new_readonly(solana_program::sysvar::rent::ID, false), // rent
            AccountMeta::new_readonly(system_program::ID, false),   // system_program
        ];
        ix_remaining_accounts
    }

    fn ix_operators(&self) -> Operators {
        let operators = vec![
            Operator::IngestInstruction(0, 4),
            Operator::IngestAccount(1),
            Operator::IngestInstructionDataSize,
        ];

        Operators { operators }
    }
}
