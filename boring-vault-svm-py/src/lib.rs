// use boring_vault_svm_core::transaction_builder;
use pyo3::prelude::*;
// use pyo3::wrap_pyfunction;
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use std::str::FromStr;
// TODO could add view functions?
// TODO also maybe the tx builder should be more of a class where you add txs to it, then you have 1 call to execute the batch, or maybe execute single?
// would need logic to break up a lot of actions into multiple batches though

// Example tx using v2 function https://solscan.io/tx/rRzu7CWxPYstBhHkKfTEdEz6fHhkDfdfuCLWKunsiCUmqWYzAhfmbQD7bZNa5FxU6BfjXF4oU8CVaezoZZQZ36t
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

    fn manage_transfer_sol_between_sub_accounts(
        &mut self,
        signer_bytes: &[u8],
        authority_bytes: Option<&[u8]>,
        vault_id: u64,
        sub_account: u8,
        to_sub_account: u8,
        amount: u64,
    ) -> PyResult<()> {
        let signer = Keypair::from_bytes(signer_bytes)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let authority = match authority_bytes {
            Some(bytes) => Some(
                Keypair::from_bytes(bytes)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
            ),
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

    fn manage_kamino_init_user_metadata(
        &mut self,
        signer_bytes: &[u8],
        authority_bytes: Option<&[u8]>,
        vault_id: u64,
        sub_account: u8,
    ) -> PyResult<()> {
        let signer = Keypair::from_bytes(signer_bytes)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let authority = match authority_bytes {
            Some(bytes) => Some(
                Keypair::from_bytes(bytes)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
            ),
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
        user_metadata: &str,
        lending_market: &str,
        tag: u8,
        id: u8,
    ) -> PyResult<()> {
        let signer = Keypair::from_bytes(signer_bytes)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let authority = match authority_bytes {
            Some(bytes) => Some(
                Keypair::from_bytes(bytes)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
            ),
            None => None,
        };

        let user_metadata_pubkey = Pubkey::from_str(user_metadata)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let lending_market_pubkey = Pubkey::from_str(lending_market)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        self.inner
            .init_obligation(
                signer,
                authority,
                vault_id,
                sub_account,
                user_metadata_pubkey,
                lending_market_pubkey,
                tag,
                id,
            )
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn manage_kamino_init_obligation_farms_for_reserve(
        &mut self,
        signer_bytes: &[u8],
        authority_bytes: Option<&[u8]>,
        vault_id: u64,
        sub_account: u8,
        obligation: &str,
        reserve: &str,
        reserve_farm_state: &str,
        obligation_farm: &str,
        lending_market: &str,
        farms_program: &str,
        mode: u8,
    ) -> PyResult<()> {
        let signer = Keypair::from_bytes(signer_bytes)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let authority = match authority_bytes {
            Some(bytes) => Some(
                Keypair::from_bytes(bytes)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
            ),
            None => None,
        };

        let obligation_pubkey = Pubkey::from_str(obligation)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let reserve_pubkey = Pubkey::from_str(reserve)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let reserve_farm_state_pubkey = Pubkey::from_str(reserve_farm_state)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let obligation_farm_pubkey = Pubkey::from_str(obligation_farm)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let lending_market_pubkey = Pubkey::from_str(lending_market)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let farms_program_pubkey = Pubkey::from_str(farms_program)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        self.inner
            .init_obligation_farms_for_reserve(
                signer,
                authority,
                vault_id,
                sub_account,
                obligation_pubkey,
                reserve_pubkey,
                reserve_farm_state_pubkey,
                obligation_farm_pubkey,
                lending_market_pubkey,
                farms_program_pubkey,
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
        reserve: &str,
        lending_market: &str,
        pyth_oracle: &str,
        switchboard_price_oracle: &str,
        switchboard_twap_oracle: &str,
        scope_prices: &str,
    ) -> PyResult<()> {
        let signer = Keypair::from_bytes(signer_bytes)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let authority = match authority_bytes {
            Some(bytes) => Some(
                Keypair::from_bytes(bytes)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
            ),
            None => None,
        };

        let reserve_pubkey = Pubkey::from_str(reserve)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let lending_market_pubkey = Pubkey::from_str(lending_market)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let pyth_oracle_pubkey = Pubkey::from_str(pyth_oracle)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let switchboard_price_oracle_pubkey = Pubkey::from_str(switchboard_price_oracle)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let switchboard_twap_oracle_pubkey = Pubkey::from_str(switchboard_twap_oracle)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let scope_prices_pubkey = Pubkey::from_str(scope_prices)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        self.inner
            .refresh_reserve(
                signer,
                authority,
                vault_id,
                sub_account,
                reserve_pubkey,
                lending_market_pubkey,
                pyth_oracle_pubkey,
                switchboard_price_oracle_pubkey,
                switchboard_twap_oracle_pubkey,
                scope_prices_pubkey,
            )
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn manage_kamino_refresh_obligation(
        &mut self,
        signer_bytes: &[u8],
        authority_bytes: Option<&[u8]>,
        vault_id: u64,
        sub_account: u8,
        lending_market: &str,
        obligation: &str,
    ) -> PyResult<()> {
        let signer = Keypair::from_bytes(signer_bytes)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let authority = match authority_bytes {
            Some(bytes) => Some(
                Keypair::from_bytes(bytes)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
            ),
            None => None,
        };

        let lending_market_pubkey = Pubkey::from_str(lending_market)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let obligation_pubkey = Pubkey::from_str(obligation)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        self.inner
            .refresh_obligation(
                signer,
                authority,
                vault_id,
                sub_account,
                lending_market_pubkey,
                obligation_pubkey,
            )
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn manage_kamino_refresh_obligation_farms_for_reserve(
        &mut self,
        signer_bytes: &[u8],
        authority_bytes: Option<&[u8]>,
        vault_id: u64,
        sub_account: u8,
        obligation: &str,
        reserve: &str,
        reserve_farm_state: &str,
        obligation_farm: &str,
        lending_market: &str,
        farms_program: &str,
        mode: u8,
    ) -> PyResult<()> {
        let signer = Keypair::from_bytes(signer_bytes)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let authority = match authority_bytes {
            Some(bytes) => Some(
                Keypair::from_bytes(bytes)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
            ),
            None => None,
        };

        let obligation_pubkey = Pubkey::from_str(obligation)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let reserve_pubkey = Pubkey::from_str(reserve)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let reserve_farm_state_pubkey = Pubkey::from_str(reserve_farm_state)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let obligation_farm_pubkey = Pubkey::from_str(obligation_farm)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let lending_market_pubkey = Pubkey::from_str(lending_market)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let farms_program_pubkey = Pubkey::from_str(farms_program)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        self.inner
            .refresh_obligation_farms_for_reserve(
                signer,
                authority,
                vault_id,
                sub_account,
                obligation_pubkey,
                reserve_pubkey,
                reserve_farm_state_pubkey,
                obligation_farm_pubkey,
                lending_market_pubkey,
                farms_program_pubkey,
                mode,
            )
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn manage_kamino_refresh_price_list(
        &mut self,
        signer_bytes: &[u8],
        authority_bytes: Option<&[u8]>,
        vault_id: u64,
        sub_account: u8,
        oracle_prices: &str,
        oracle_mapping: &str,
        oracle_twaps: &str,
        price_accounts: Vec<String>,
        tokens: Vec<u16>,
    ) -> PyResult<()> {
        let signer = Keypair::from_bytes(signer_bytes)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let authority = match authority_bytes {
            Some(bytes) => Some(
                Keypair::from_bytes(bytes)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
            ),
            None => None,
        };

        let oracle_prices_pubkey = Pubkey::from_str(oracle_prices)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let oracle_mapping_pubkey = Pubkey::from_str(oracle_mapping)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let oracle_twaps_pubkey = Pubkey::from_str(oracle_twaps)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let price_accounts_pubkeys: Vec<Pubkey> = price_accounts
            .iter()
            .map(|s| Pubkey::from_str(s).unwrap())
            .collect();

        self.inner
            .refresh_price_list(
                signer,
                authority,
                vault_id,
                sub_account,
                oracle_prices_pubkey,
                oracle_mapping_pubkey,
                oracle_twaps_pubkey,
                price_accounts_pubkeys,
                tokens,
            )
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn manage_kamino_deposit(
        &mut self,
        signer_bytes: &[u8],
        authority_bytes: Option<&[u8]>,
        vault_id: u64,
        sub_account: u8,
        lending_market: &str,
        obligation: &str,
        reserve: &str,
        reserve_liquidity_mint: &str,
        reserve_liquidity_supply: &str,
        reserve_collateral_mint: &str,
        reserve_destination_deposit_collateral: &str,
        amount: u64,
    ) -> PyResult<()> {
        let signer = Keypair::from_bytes(signer_bytes)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let authority = match authority_bytes {
            Some(bytes) => Some(
                Keypair::from_bytes(bytes)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
            ),
            None => None,
        };

        let lending_market_pubkey = Pubkey::from_str(lending_market)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let obligation_pubkey = Pubkey::from_str(obligation)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let reserve_pubkey = Pubkey::from_str(reserve)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let reserve_liquidity_mint_pubkey = Pubkey::from_str(reserve_liquidity_mint)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let reserve_liquidity_supply_pubkey = Pubkey::from_str(reserve_liquidity_supply)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let reserve_collateral_mint_pubkey = Pubkey::from_str(reserve_collateral_mint)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let reserve_destination_deposit_collateral_pubkey =
            Pubkey::from_str(reserve_destination_deposit_collateral)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        self.inner
            .kamino_deposit(
                signer,
                authority,
                vault_id,
                sub_account,
                lending_market_pubkey,
                obligation_pubkey,
                reserve_pubkey,
                reserve_liquidity_mint_pubkey,
                reserve_liquidity_supply_pubkey,
                reserve_collateral_mint_pubkey,
                reserve_destination_deposit_collateral_pubkey,
                amount,
            )
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn manage_deposit_solend(
        &mut self,
        signer_bytes: &[u8],
        authority_bytes: Option<&[u8]>,
        vault_id: u64,
        sub_account: u8,
        deposit_mint: &str,
        reserve_collateral_mint: &str,
        lending_market: &str,
        reserve: &str,
        reserve_liquidity_supply_spl_token_account: &str,
        lending_market_authority: &str,
        destination_deposit_reserve_collateral_supply_spl_token_account: &str,
        pyth_price_oracle: &str,
        switchboard_price_oracle: &str,
        amount: u64,
    ) -> PyResult<()> {
        let signer = Keypair::from_bytes(signer_bytes)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let authority = match authority_bytes {
            Some(bytes) => Some(
                Keypair::from_bytes(bytes)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
            ),
            None => None,
        };

        let deposit_mint_pubkey = Pubkey::from_str(deposit_mint)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let reserve_collateral_mint_pubkey = Pubkey::from_str(reserve_collateral_mint)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let lending_market_pubkey = Pubkey::from_str(lending_market)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let reserve_pubkey = Pubkey::from_str(reserve)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let reserve_liquidity_supply_pubkey =
            Pubkey::from_str(reserve_liquidity_supply_spl_token_account)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let lending_market_authority_pubkey = Pubkey::from_str(lending_market_authority)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let destination_deposit_pubkey =
            Pubkey::from_str(destination_deposit_reserve_collateral_supply_spl_token_account)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let pyth_oracle_pubkey = Pubkey::from_str(pyth_price_oracle)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let switchboard_oracle_pubkey = Pubkey::from_str(switchboard_price_oracle)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

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
        let signer = Keypair::from_bytes(signer_bytes)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let authority = match authority_bytes {
            Some(bytes) => Some(
                Keypair::from_bytes(bytes)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
            ),
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
        let signer = Keypair::from_bytes(signer_bytes)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let authority = match authority_bytes {
            Some(bytes) => Some(
                Keypair::from_bytes(bytes)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
            ),
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
        let signer = Keypair::from_bytes(signer_bytes)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let authority = match authority_bytes {
            Some(bytes) => Some(
                Keypair::from_bytes(bytes)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
            ),
            None => None,
        };

        self.inner
            .mint_jito_sol(signer, authority, vault_id, sub_account, amount)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }
}

#[pymodule]
fn boring_vault_svm(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<TransactionBuilder>()?;
    Ok(())
}
