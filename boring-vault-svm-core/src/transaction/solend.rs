use eyre::Result;
use solana_pubkey::Pubkey;

use crate::{instructions::create_deposit_solend_instructions, KeypairOrPublickey};

use crate::builder::Builder;

impl Builder {
    pub fn deposit_solend(
        &mut self,
        signer: KeypairOrPublickey,
        authority: Option<KeypairOrPublickey>,
        vault_id: u64,
        sub_account: u8,
        deposit_mint: Pubkey,
        reserve_collateral_mint: Pubkey,
        lending_market: Pubkey,
        reserve: Pubkey,
        reserve_liquidity_supply_spl_token_account: Pubkey,
        lending_market_authority: Pubkey,
        destination_deposit_reserve_collateral_supply_spl_token_account: Pubkey,
        pyth_price_oracle: Pubkey,
        switchboard_price_oracle: Pubkey,
        amount: u64,
    ) -> Result<()> {
        let ixs = create_deposit_solend_instructions(
            &self.client,
            &signer,
            authority.as_ref(),
            vault_id,
            sub_account,
            &deposit_mint,
            &reserve_collateral_mint,
            &lending_market,
            &reserve,
            &reserve_liquidity_supply_spl_token_account,
            &lending_market_authority,
            &destination_deposit_reserve_collateral_supply_spl_token_account,
            &pyth_price_oracle,
            &switchboard_price_oracle,
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
}
