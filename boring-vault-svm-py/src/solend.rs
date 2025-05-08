use boring_vault_svm_core::KeypairOrPublickey;
use pyo3::{pymethods, PyErr, PyResult};

use crate::{
    utils::{to_keypair_from_bytes, to_pubkey_from_string},
    Builder,
};

#[pymethods]
impl Builder {
    fn manage_deposit_solend(
        &mut self,
        signer_bytes: &[u8],
        authority_bytes: Option<&[u8]>,
        vault_id: u64,
        sub_account: u8,
        deposit_mint: String,
        reserve_collateral_mint: String,
        lending_market: String,
        reserve: String,
        reserve_liquidity_supply_spl_token_account: String,
        lending_market_authority: String,
        destination_deposit_reserve_collateral_supply_spl_token_account: String,
        pyth_price_oracle: String,
        switchboard_price_oracle: String,
        amount: u64,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        let authority = match authority_bytes {
            Some(bytes) => Some(KeypairOrPublickey::Keypair(to_keypair_from_bytes(bytes)?)),
            None => None,
        };

        let deposit_mint_pubkey = to_pubkey_from_string(deposit_mint)?;
        let reserve_collateral_mint_pubkey = to_pubkey_from_string(reserve_collateral_mint)?;
        let lending_market_pubkey = to_pubkey_from_string(lending_market)?;
        let reserve_pubkey = to_pubkey_from_string(reserve)?;
        let reserve_liquidity_supply_pubkey =
            to_pubkey_from_string(reserve_liquidity_supply_spl_token_account)?;
        let lending_market_authority_pubkey = to_pubkey_from_string(lending_market_authority)?;
        let destination_deposit_pubkey =
            to_pubkey_from_string(destination_deposit_reserve_collateral_supply_spl_token_account)?;
        let pyth_oracle_pubkey = to_pubkey_from_string(pyth_price_oracle)?;
        let switchboard_oracle_pubkey = to_pubkey_from_string(switchboard_price_oracle)?;

        self.inner
            .deposit_solend(
                signer,
                authority,
                vault_id,
                sub_account,
                deposit_mint_pubkey,
                reserve_collateral_mint_pubkey,
                lending_market_pubkey,
                reserve_pubkey,
                reserve_liquidity_supply_pubkey,
                lending_market_authority_pubkey,
                destination_deposit_pubkey,
                pyth_oracle_pubkey,
                switchboard_oracle_pubkey,
                amount,
            )
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }
}
