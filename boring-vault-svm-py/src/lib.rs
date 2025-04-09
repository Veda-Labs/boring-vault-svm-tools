// use boring_vault_svm_core::transaction_builder;
use pyo3::prelude::*;
// use pyo3::wrap_pyfunction;
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use std::str::FromStr;
// TODO could add view functions?
// TODO also maybe the tx builder should be more of a class where you add txs to it, then you have 1 call to execute the batch, or maybe execute single?
// would need logic to break up a lot of actions into multiple batches though

#[pyclass]
struct TransactionBuilder {
    inner: boring_vault_svm_core::transaction_builder::TransactionBuilder,
}

#[pymethods]
impl TransactionBuilder {
    #[new]
    fn new(rpc_url: String) -> Self {
        Self {
            inner: boring_vault_svm_core::transaction_builder::TransactionBuilder::new(rpc_url),
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
        let authority = Pubkey::from_str(&authority)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let signer = Keypair::from_bytes(signer_bytes)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let program_signer = Keypair::from_bytes(program_signer_bytes)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

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
        let authority = Pubkey::from_str(&authority)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let signer = Keypair::from_bytes(signer_bytes)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let base_asset = Pubkey::from_str(&base_asset)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        // Convert Option<String> to Option<Pubkey>
        let exchange_rate_provider = match exchange_rate_provider {
            Some(s) => Some(
                Pubkey::from_str(&s)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
            ),
            None => None,
        };

        let payout_address = match payout_address {
            Some(s) => Some(
                Pubkey::from_str(&s)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
            ),
            None => None,
        };

        let withdraw_authority = match withdraw_authority {
            Some(s) => Some(
                Pubkey::from_str(&s)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
            ),
            None => None,
        };

        let strategist = match strategist {
            Some(s) => Some(
                Pubkey::from_str(&s)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
            ),
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
        let signer = Keypair::from_bytes(signer_bytes)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let mint = Pubkey::from_str(&mint)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let price_feed = Pubkey::from_str(&price_feed)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

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

    fn deposit_sol(
        &mut self,
        signer_bytes: &[u8],
        vault_id: u64,
        user_pubkey: String,
        deposit_amount: u64,
        min_mint_amount: u64,
    ) -> PyResult<()> {
        let signer = Keypair::from_bytes(signer_bytes)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let user_pubkey = Pubkey::from_str(&user_pubkey)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        self.inner
            .deposit_sol(
                signer,
                vault_id,
                user_pubkey,
                deposit_amount,
                min_mint_amount,
            )
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }
}

#[pymodule]
fn boring_vault_svm(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<TransactionBuilder>()?;
    Ok(())
}
