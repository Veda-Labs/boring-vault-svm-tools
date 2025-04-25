pub mod system;
pub mod jito;
pub mod kamino;
pub mod solend;

pub use jito::*;
pub use solend::*;
pub use kamino::*;
pub use system::*;

use solana_instruction::account_meta::AccountMeta;
use solana_pubkey::Pubkey;
use crate::utils::bindings::boring_vault_svm::types::Operators;

pub trait ExternalInstruction {
    fn vault_id(&self) -> u64;
    fn sub_account(&self) -> u8;
    fn ix_program_id(&self) -> Pubkey;
    fn ix_data(&self) -> Vec<u8>;
    fn ix_remaining_accounts(&self) -> Vec<AccountMeta>;
    fn ix_operators(&self) -> Operators;
}

