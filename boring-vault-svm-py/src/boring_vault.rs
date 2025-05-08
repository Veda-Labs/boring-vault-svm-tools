use boring_vault_svm_core::KeypairOrPublickey;
use pyo3::{pymethods, PyErr, PyResult};

use crate::{
    utils::{to_keypair_from_bytes, to_pubkey_from_string},
    Builder,
};

#[pymethods]
impl Builder {
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
}
