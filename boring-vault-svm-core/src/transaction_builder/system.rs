use crate::{
    instructions::{create_unwrap_sol_instructions, create_wrap_sol_instructions},
    KeypairOrPublickey,
};
use eyre::Result;

use super::TransactionBuilder;

impl TransactionBuilder {
    pub fn wrap_sol(
        &mut self,
        signer: KeypairOrPublickey,
        authority: Option<KeypairOrPublickey>,
        vault_id: u64,
        sub_account: u8,
        amount: u64,
    ) -> Result<()> {
        let ixs = create_wrap_sol_instructions(
            &self.client,
            &signer,
            authority.as_ref(),
            vault_id,
            sub_account,
            amount,
        )?;

        for ix in ixs {
            self.instructions.push(ix);
        }

        self.add_signer_if_keypair(signer);
        if let Some(authority) = authority {
            self.add_signer_if_keypair(authority);
        }

        Ok(())
    }

    pub fn unwrap_sol(
        &mut self,
        signer: KeypairOrPublickey,
        authority: Option<KeypairOrPublickey>,
        vault_id: u64,
        sub_account: u8,
    ) -> Result<()> {
        let ixs = create_unwrap_sol_instructions(
            &self.client,
            &signer,
            authority.as_ref(),
            vault_id,
            sub_account,
        )?;

        for ix in ixs {
            self.instructions.push(ix);
        }

        self.add_signer_if_keypair(signer);
        if let Some(authority) = authority {
            self.add_signer_if_keypair(authority);
        }

        Ok(())
    }
}
