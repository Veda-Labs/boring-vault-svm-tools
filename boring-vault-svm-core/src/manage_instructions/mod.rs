pub mod jito;
pub mod kamino;
pub mod solend;
pub mod system;

pub use jito::*;
pub use kamino::*;
pub use solend::*;
pub use system::*;

use crate::utils::bindings::boring_vault_svm::types::Operators;
use solana_instruction::{account_meta::AccountMeta, Instruction};
use solana_pubkey::Pubkey;

pub trait ExternalInstruction {
    fn vault_id(&self) -> u64;
    fn sub_account(&self) -> u8;
    fn ix_program_id(&self) -> Pubkey;
    fn ix_data(&self) -> Vec<u8>;
    fn ix_remaining_accounts(&self) -> Vec<AccountMeta>;
    fn ix_operators(&self) -> Operators;
    fn to_instruction(&self) -> Instruction;
}

#[macro_export]
macro_rules! impl_external_instruction_common {
    () => {
        fn vault_id(&self) -> u64 {
            self.vault_id
        }
        fn sub_account(&self) -> u8 {
            self.sub_account
        }
        fn to_instruction(&self) -> solana_instruction::Instruction {
            solana_instruction::Instruction {
                program_id: self.ix_program_id(),
                accounts: self.ix_remaining_accounts(),
                data: self.ix_data(),
            }
        }
    };
}
