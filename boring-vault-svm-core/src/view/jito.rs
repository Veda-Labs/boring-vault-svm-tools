use crate::{
    builder::Builder,
    manage_instructions::{ExternalInstruction, MintJitoSol},
};
use eyre::Result;
use solana_pubkey::Pubkey;

impl Builder {
    pub fn get_jito_digest(&self, vault_id: u64, sub_account: u8) -> Result<(Pubkey, String)> {
        let ix = MintJitoSol::new(vault_id, sub_account, 0);

        Ok(ix.get_digest())
    }
}
