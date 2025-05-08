#![allow(clippy::too_many_arguments)]
use boring_vault_svm_core::KeypairOrPublickey;
// use boring_vault_svm_core::builder_builder;
use pyo3::prelude::*;
// use pyo3::wrap_pyfunction;
use solana_keypair::Keypair;
use utils::{to_keypair_from_bytes, to_pubkey_from_string};

mod utils;
// TODO could add view functions?
// TODO also maybe the tx builder should be more of a class where you add txs to it, then you have 1 call to execute the batch, or maybe execute single?
// would need logic to break up a lot of actions into multiple batches though

// Example tx using v2 function https://solscan.io/tx/rRzu7CWxPYstBhHkKfTEdEz6fHhkDfdfuCLWKunsiCUmqWYzAhfmbQD7bZNa5FxU6BfjXF4oU8CVaezoZZQZ36t
#[pyclass]
struct Builder {
    inner: boring_vault_svm_core::builder::Builder,
}

#[pymethods]
impl Builder {
    #[new]
    fn new(rpc_url: String, path_root: Option<String>) -> Self {
        Self {
            inner: boring_vault_svm_core::builder::Builder::new(rpc_url, path_root),
        }
    }

    fn try_bundle_all(&mut self, payer_bytes: &[u8]) -> PyResult<String> {
        let payer = Keypair::from_bytes(payer_bytes)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let tx_hash = self
            .inner
            .try_bundle_all(payer)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(tx_hash)
    }

    fn initialize(
        &mut self,
        authority: String,
        signer_bytes: &[u8],
        program_signer_bytes: &[u8],
    ) -> PyResult<()> {
        let authority = to_pubkey_from_string(authority)?;
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);
        let program_signer: KeypairOrPublickey =
            KeypairOrPublickey::Keypair(to_keypair_from_bytes(program_signer_bytes)?);

        self.inner
            .initialize(authority, signer, program_signer)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn deploy(
        &mut self,
        authority: String,
        signer_bytes: &[u8],
        base_asset: String,
        name: String,
        symbol: String,
        exchange_rate_provider: Option<String>,
        exchange_rate: u64,
        payout_address: Option<String>,
        allowed_exchange_rate_change_upper_bound: u16,
        allowed_exchange_rate_change_lower_bound: u16,
        minimum_update_delay_in_seconds: u32,
        platform_fee_bps: Option<u16>,
        performance_fee_bps: Option<u16>,
        withdraw_authority: Option<String>,
        strategist: Option<String>,
    ) -> PyResult<()> {
        let authority = to_pubkey_from_string(authority)?;
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);
        let base_asset = to_pubkey_from_string(base_asset)?;

        let exchange_rate_provider = match exchange_rate_provider {
            Some(s) => Some(to_pubkey_from_string(s)?),
            None => None,
        };

        let payout_address = match payout_address {
            Some(s) => Some(to_pubkey_from_string(s)?),
            None => None,
        };

        let withdraw_authority = match withdraw_authority {
            Some(s) => Some(to_pubkey_from_string(s)?),
            None => None,
        };

        let strategist = match strategist {
            Some(s) => Some(to_pubkey_from_string(s)?),
            None => None,
        };

        self.inner
            .deploy(
                authority,
                signer,
                base_asset,
                name,
                symbol,
                exchange_rate_provider,
                exchange_rate,
                payout_address,
                allowed_exchange_rate_change_upper_bound,
                allowed_exchange_rate_change_lower_bound,
                minimum_update_delay_in_seconds,
                platform_fee_bps,
                performance_fee_bps,
                withdraw_authority,
                strategist,
            )
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn update_asset_data(
        &mut self,
        signer_bytes: &[u8],
        vault_id: u64,
        mint: String,
        allow_deposits: bool,
        allow_withdrawals: bool,
        share_premium_bps: u16,
        is_pegged_to_base_asset: bool,
        price_feed: String,
        inverse_price_feed: bool,
        max_staleness: u64,
        min_samples: u32,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);
        let mint = to_pubkey_from_string(mint)?;
        let price_feed = to_pubkey_from_string(price_feed)?;

        self.inner
            .update_asset_data(
                signer,
                vault_id,
                mint,
                allow_deposits,
                allow_withdrawals,
                share_premium_bps,
                is_pegged_to_base_asset,
                price_feed,
                inverse_price_feed,
                max_staleness,
                min_samples,
            )
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn pause(&mut self, signer_bytes: &[u8], vault_id: u64) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        self.inner
            .pause(signer, vault_id)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn unpause(&mut self, signer_bytes: &[u8], vault_id: u64) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        self.inner
            .unpause(signer, vault_id)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn transfer_authority(
        &mut self,
        signer_bytes: &[u8],
        vault_id: u64,
        pending_authority: String,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);
        let pending_authority_pubkey = to_pubkey_from_string(pending_authority)?;

        self.inner
            .transfer_authority(signer, vault_id, pending_authority_pubkey)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn accept_authority(&mut self, signer_bytes: &[u8], vault_id: u64) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        self.inner
            .accept_authority(signer, vault_id)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn get_lend_digest(
        &mut self,
        vault_id: u64,
        sub_account: u8,
        tag: u8,
        id: u8,
    ) -> PyResult<(String, String)> {
        let (address, digest) = self
            .inner
            .get_lend_digest(vault_id, sub_account, tag, id)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok((address.to_string(), digest))
    }

    fn get_borrow_digest(
        &mut self,
        vault_id: u64,
        sub_account: u8,
        tag: u8,
        id: u8,
    ) -> PyResult<(String, String)> {
        let (address, digest) = self
            .inner
            .get_borrow_digest(vault_id, sub_account, tag, id)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok((address.to_string(), digest))
    }

    fn get_jito_digest(&mut self, vault_id: u64, sub_account: u8) -> PyResult<()> {
        self.inner
            .get_jito_digest(vault_id, sub_account)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn close_cpi_digest(
        &mut self,
        signer_bytes: &[u8],
        vault_id: u64,
        digest: [u8; 32],
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        self.inner
            .close_cpi_digest(signer, vault_id, digest)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn update_exchange_rate_provider(
        &mut self,
        signer_bytes: &[u8],
        vault_id: u64,
        new_provider: String,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);
        let new_provider_pubkey = to_pubkey_from_string(new_provider)?;

        self.inner
            .update_exchange_rate_provider(signer, vault_id, new_provider_pubkey)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn set_withdraw_authority(
        &mut self,
        signer_bytes: &[u8],
        vault_id: u64,
        new_authority: String,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);
        let new_authority_pubkey = to_pubkey_from_string(new_authority)?;

        self.inner
            .set_withdraw_authority(signer, vault_id, new_authority_pubkey)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn set_payout(
        &mut self,
        signer_bytes: &[u8],
        vault_id: u64,
        new_payout: String,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);
        let new_payout_pubkey = to_pubkey_from_string(new_payout)?;

        self.inner
            .set_payout(signer, vault_id, new_payout_pubkey)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn configure_exchange_rate_update_bounds(
        &mut self,
        signer_bytes: &[u8],
        vault_id: u64,
        upper_bound: u16,
        lower_bound: u16,
        minimum_update_delay: u32,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        self.inner
            .configure_exchange_rate_update_bounds(
                signer,
                vault_id,
                upper_bound,
                lower_bound,
                minimum_update_delay,
            )
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn set_fees(
        &mut self,
        signer_bytes: &[u8],
        vault_id: u64,
        platform_fee_bps: u16,
        performance_fee_bps: u16,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        self.inner
            .set_fees(signer, vault_id, platform_fee_bps, performance_fee_bps)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn set_strategist(
        &mut self,
        signer_bytes: &[u8],
        vault_id: u64,
        new_strategist: String,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);
        let new_strategist_pubkey = to_pubkey_from_string(new_strategist)?;

        self.inner
            .set_strategist(signer, vault_id, new_strategist_pubkey)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn claim_fees_in_base(
        &mut self,
        signer_bytes: &[u8],
        vault_id: u64,
        sub_account: u8,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        self.inner
            .claim_fees_in_base(signer, vault_id, sub_account)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn deposit_sol(
        &mut self,
        signer_bytes: &[u8],
        vault_id: u64,
        deposit_amount: u64,
        min_mint_amount: u64,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        self.inner
            .deposit_sol(signer, vault_id, deposit_amount, min_mint_amount)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn deposit(
        &mut self,
        signer_bytes: &[u8],
        vault_id: u64,
        deposit_mint: String,
        deposit_amount: u64,
        min_mint_amount: u64,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);
        let deposit_mint_pubkey = to_pubkey_from_string(deposit_mint)?;

        self.inner
            .deposit(
                signer,
                vault_id,
                deposit_mint_pubkey,
                deposit_amount,
                min_mint_amount,
            )
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn withdraw(
        &mut self,
        signer_bytes: &[u8],
        vault_id: u64,
        withdraw_mint: String,
        share_amount: u64,
        min_asset_amount: u64,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);
        let withdraw_mint_pubkey = to_pubkey_from_string(withdraw_mint)?;

        self.inner
            .withdraw(
                signer,
                vault_id,
                withdraw_mint_pubkey,
                share_amount,
                min_asset_amount,
            )
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn update_exchange_rate(
        &mut self,
        signer_bytes: &[u8],
        vault_id: u64,
        new_exchange_rate: u64,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        self.inner
            .update_exchange_rate(signer, vault_id, new_exchange_rate)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn manage_transfer_sol_between_sub_accounts(
        &mut self,
        signer_bytes: &[u8],
        authority_bytes: Option<&[u8]>,
        vault_id: u64,
        sub_account: u8,
        to_sub_account: u8,
        amount: u64,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        let authority = match authority_bytes {
            Some(bytes) => Some(KeypairOrPublickey::Keypair(to_keypair_from_bytes(bytes)?)),
            None => None,
        };

        self.inner
            .transfer_sol_between_sub_accounts(
                signer,
                authority,
                vault_id,
                sub_account,
                to_sub_account,
                amount,
            )
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn set_deposit_sub_account(
        &mut self,
        signer_bytes: &[u8],
        vault_id: u64,
        new_sub_account: u8,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        self.inner
            .set_deposit_sub_account(signer, vault_id, new_sub_account)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn set_withdraw_sub_account(
        &mut self,
        signer_bytes: &[u8],
        vault_id: u64,
        new_sub_account: u8,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        self.inner
            .set_withdraw_sub_account(signer, vault_id, new_sub_account)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn manage_kamino_init_user_metadata(
        &mut self,
        signer_bytes: &[u8],
        authority_bytes: Option<&[u8]>,
        vault_id: u64,
        sub_account: u8,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        let authority = match authority_bytes {
            Some(bytes) => Some(KeypairOrPublickey::Keypair(to_keypair_from_bytes(bytes)?)),
            None => None,
        };

        self.inner
            .init_user_metadata(signer, authority, vault_id, sub_account)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn manage_kamino_init_obligation(
        &mut self,
        signer_bytes: &[u8],
        authority_bytes: Option<&[u8]>,
        vault_id: u64,
        sub_account: u8,
        tag: u8,
        id: u8,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        let authority = match authority_bytes {
            Some(bytes) => Some(KeypairOrPublickey::Keypair(to_keypair_from_bytes(bytes)?)),
            None => None,
        };

        self.inner
            .init_obligation(signer, authority, vault_id, sub_account, tag, id)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn manage_kamino_init_obligation_farms_for_reserve(
        &mut self,
        signer_bytes: &[u8],
        authority_bytes: Option<&[u8]>,
        vault_id: u64,
        sub_account: u8,
        tag: Option<u8>,
        id: Option<u8>,
        mode: u8,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        let authority = match authority_bytes {
            Some(bytes) => Some(KeypairOrPublickey::Keypair(to_keypair_from_bytes(bytes)?)),
            None => None,
        };

        self.inner
            .init_obligation_farms_for_reserve(
                signer,
                authority,
                vault_id,
                sub_account,
                tag,
                id,
                mode,
            )
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn manage_kamino_refresh_reserve(
        &mut self,
        signer_bytes: &[u8],
        authority_bytes: Option<&[u8]>,
        vault_id: u64,
        sub_account: u8,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        let authority = match authority_bytes {
            Some(bytes) => Some(KeypairOrPublickey::Keypair(to_keypair_from_bytes(bytes)?)),
            None => None,
        };

        self.inner
            .refresh_reserves(signer, authority, vault_id, sub_account)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn manage_kamino_refresh_obligation(
        &mut self,
        signer_bytes: &[u8],
        authority_bytes: Option<&[u8]>,
        vault_id: u64,
        sub_account: u8,
        tag: u8,
        id: u8,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        let authority = match authority_bytes {
            Some(bytes) => Some(KeypairOrPublickey::Keypair(to_keypair_from_bytes(bytes)?)),
            None => None,
        };

        self.inner
            .refresh_obligation(signer, authority, vault_id, sub_account, tag, id)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn manage_kamino_refresh_obligation_farms_for_reserve(
        &mut self,
        signer_bytes: &[u8],
        authority_bytes: Option<&[u8]>,
        vault_id: u64,
        sub_account: u8,
        tag: u8,
        id: u8,
        mode: u8,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        let authority = match authority_bytes {
            Some(bytes) => Some(KeypairOrPublickey::Keypair(to_keypair_from_bytes(bytes)?)),
            None => None,
        };

        self.inner
            .refresh_obligation_farms_for_reserve(
                signer,
                authority,
                vault_id,
                sub_account,
                tag,
                id,
                mode,
            )
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn manage_kamino_refresh_price_list(
        &mut self,
        signer_bytes: &[u8],
        vault_id: u64,
        sub_account: u8,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        self.inner
            .refresh_price_list(signer, vault_id, sub_account)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn manage_kamino_deposit(
        &mut self,
        signer_bytes: &[u8],
        authority_bytes: Option<&[u8]>,
        vault_id: u64,
        sub_account: u8,
        tag: u8,
        id: u8,
        amount: u64,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        let authority = match authority_bytes {
            Some(bytes) => Some(KeypairOrPublickey::Keypair(to_keypair_from_bytes(bytes)?)),
            None => None,
        };

        self.inner
            .kamino_deposit(signer, authority, vault_id, sub_account, tag, id, amount)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn manage_kamino_borrow(
        &mut self,
        signer_bytes: &[u8],
        authority_bytes: Option<&[u8]>,
        vault_id: u64,
        sub_account: u8,
        tag: u8,
        id: u8,
        amount: u64,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        let authority = match authority_bytes {
            Some(bytes) => Some(KeypairOrPublickey::Keypair(to_keypair_from_bytes(bytes)?)),
            None => None,
        };

        self.inner
            .kamino_borrow(signer, authority, vault_id, sub_account, tag, id, amount)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

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

    fn manage_wrap_sol(
        &mut self,
        signer_bytes: &[u8],
        authority_bytes: Option<&[u8]>,
        vault_id: u64,
        sub_account: u8,
        amount: u64,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        let authority = match authority_bytes {
            Some(bytes) => Some(KeypairOrPublickey::Keypair(to_keypair_from_bytes(bytes)?)),
            None => None,
        };

        self.inner
            .wrap_sol(signer, authority, vault_id, sub_account, amount)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn manage_unwrap_sol(
        &mut self,
        signer_bytes: &[u8],
        authority_bytes: Option<&[u8]>,
        vault_id: u64,
        sub_account: u8,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        let authority = match authority_bytes {
            Some(bytes) => Some(KeypairOrPublickey::Keypair(to_keypair_from_bytes(bytes)?)),
            None => None,
        };

        self.inner
            .unwrap_sol(signer, authority, vault_id, sub_account)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn manage_mint_jito_sol(
        &mut self,
        signer_bytes: &[u8],
        authority_bytes: Option<&[u8]>,
        vault_id: u64,
        sub_account: u8,
        amount: u64,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        let authority = match authority_bytes {
            Some(bytes) => Some(KeypairOrPublickey::Keypair(to_keypair_from_bytes(bytes)?)),
            None => None,
        };

        self.inner
            .mint_jito_sol(signer, authority, vault_id, sub_account, amount)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn lend(
        &mut self,
        signer_bytes: &[u8],
        authority_bytes: &[u8],
        vault_id: u64,
        sub_account: u8,
        amount: u64,
        tag: u8,
        id: u8,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        let authority = KeypairOrPublickey::Keypair(to_keypair_from_bytes(authority_bytes)?);

        self.inner
            .lend(
                signer,
                Some(authority),
                vault_id,
                sub_account,
                amount,
                tag,
                id,
            )
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn borrow(
        &mut self,
        signer_bytes: &[u8],
        authority_bytes: &[u8],
        vault_id: u64,
        sub_account: u8,
        amount: u64,
        tag: u8,
        id: u8,
    ) -> PyResult<()> {
        let signer = KeypairOrPublickey::Keypair(to_keypair_from_bytes(signer_bytes)?);

        let authority = KeypairOrPublickey::Keypair(to_keypair_from_bytes(authority_bytes)?);

        self.inner
            .borrow(
                signer,
                Some(authority),
                vault_id,
                sub_account,
                amount,
                tag,
                id,
            )
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }
}

#[pymodule]
fn boring_vault_svm(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Builder>()?;
    Ok(())
}
