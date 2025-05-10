use eyre::Result;
use solana_pubkey::Pubkey;

use crate::{
    builder::Builder,
    manage_instructions::{ExternalInstruction, KaminoBorrow, KaminoDeposit},
    state::{Obligation, Reserve},
    utils::get_account_data_unsafe,
};

impl Builder {
    pub fn get_lend_digest(
        &self,
        vault_id: u64,
        sub_account: u8,
        tag: u8,
        id: u8,
    ) -> Result<(Pubkey, String)> {
        let lend = KaminoDeposit::new(
            vault_id,
            sub_account,
            self.kamino_config.lending_market,
            self.kamino_config.lend.reserve,
            self.kamino_config.lend.reserve_liquidity_mint,
            self.kamino_config.lend.reserve_liquidity_supply,
            self.kamino_config.lend.reserve_collateral_mint,
            self.kamino_config
                .lend
                .reserve_destination_deposit_collateral,
            self.kamino_config.lend.reserve_farm_state,
            tag,
            id,
            0,
        );

        Ok(lend.get_digest())
    }

    pub fn get_borrow_digest(
        &self,
        vault_id: u64,
        sub_account: u8,
        tag: u8,
        id: u8,
    ) -> Result<(Pubkey, String)> {
        let borrow = KaminoBorrow::new(
            vault_id,
            sub_account,
            self.kamino_config.lending_market,
            self.kamino_config.borrow.reserve,
            self.kamino_config.borrow.reserve_source_liquidity_mint,
            self.kamino_config.borrow.reserve_source_liquidity,
            self.kamino_config
                .borrow
                .reserve_source_liquidity_fee_receiver,
            tag,
            id,
            0,
        );

        Ok(borrow.get_digest())
    }

    pub fn get_reserve(&self, reserve: &Pubkey) -> Result<Reserve> {
        get_account_data_unsafe::<Reserve>(&self.client, reserve)
    }

    pub fn get_obligation(&self, obligation: &Pubkey) -> Result<Obligation> {
        get_account_data_unsafe::<Obligation>(&self.client, obligation)
    }
}
