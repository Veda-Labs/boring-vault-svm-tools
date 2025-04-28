use crate::manage_instructions::ExternalInstruction;
use crate::utils::bindings::boring_vault_svm::types::{Operator, Operators};
use crate::utils::pdas;
use solana_instruction::account_meta::AccountMeta;
use solana_program::system_instruction;
use solana_program::system_program;
use solana_pubkey::Pubkey;

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

pub struct CreateAccountWithSeed {
    vault_id: u64,
    sub_account: u8,
    seed_str: String,
    lamports: u64,
    space: u64,
    owner: Pubkey,
}

impl CreateAccountWithSeed {
    pub fn new(
        vault_id: u64,
        sub_account: u8,
        seed_str: String,
        lamports: u64,
        space: u64,
        owner: Pubkey,
    ) -> Self {
        Self {
            vault_id,
            sub_account,
            seed_str,
            lamports,
            space,
            owner,
        }
    }
}

impl ExternalInstruction for CreateAccountWithSeed {
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
        let vault_pda = pdas::get_vault_pda(self.vault_id, self.sub_account);
        let pda = Pubkey::create_with_seed(&vault_pda, &self.seed_str, &self.owner)
            .expect("Failed to create account address with seed");

        // Use the system instruction helper to generate the data
        let ix = system_instruction::create_account_with_seed(
            &vault_pda,     // from (funding account)
            &pda,           // to (the account to create)
            &vault_pda,     // base
            &self.seed_str, // seed string
            self.lamports,  // amount of lamports
            self.space,     // amount of space in bytes
            &self.owner,    // owner of the created account
        );
        ix.data
    }

    fn ix_remaining_accounts(&self) -> Vec<AccountMeta> {
        let vault_pda = pdas::get_vault_pda(self.vault_id, self.sub_account);
        let pda = Pubkey::create_with_seed(&vault_pda, &self.seed_str, &self.owner)
            .expect("Failed to create account address with seed");

        vec![
            AccountMeta::new(vault_pda, false),
            AccountMeta::new(pda, false),
        ]
    }

    fn ix_operators(&self) -> Operators {
        let operators = vec![
            Operator::IngestInstruction(0, 4),
            Operator::IngestAccount(0),
            Operator::IngestInstructionDataSize,
        ];

        Operators { operators }
    }
}

// unwrapping
// call close account
pub struct CloseAccount {
    vault_id: u64,
    sub_account: u8,
    account: Pubkey,
    token_program: Pubkey,
}

impl CloseAccount {
    pub fn new(vault_id: u64, sub_account: u8, account: Pubkey, token_program: Pubkey) -> Self {
        Self {
            vault_id,
            sub_account,
            account,
            token_program,
        }
    }
}

impl ExternalInstruction for CloseAccount {
    fn vault_id(&self) -> u64 {
        self.vault_id
    }

    fn sub_account(&self) -> u8 {
        self.sub_account
    }

    fn ix_program_id(&self) -> Pubkey {
        self.token_program
    }

    fn ix_data(&self) -> Vec<u8> {
        vec![9] // 9 is the discriminator for closing an account.
    }

    fn ix_remaining_accounts(&self) -> Vec<AccountMeta> {
        let vault_pda = pdas::get_vault_pda(self.vault_id, self.sub_account);

        vec![
            AccountMeta::new(self.account, false),       // Account to close
            AccountMeta::new(vault_pda, false),          // destination
            AccountMeta::new_readonly(vault_pda, false), // owner
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
