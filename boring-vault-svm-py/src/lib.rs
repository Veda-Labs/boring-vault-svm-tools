use boring_vault_svm_core::transaction_builder;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use std::str::FromStr;
// TODO could add view functions?
// TODO also maybe the tx builder should be more of a class where you add txs to it, then you have 1 call to execute the batch, or maybe execute single?
// would need logic to break up a lot of actions into multiple batches though
// TODO return tx hash?
#[pyfunction]
fn initialize(
    authority: String,
    signer_bytes: &[u8],
    program_signer_bytes: &[u8],
) -> PyResult<String> {
    let builder = transaction_builder::TransactionBuilder::new_local();
    let authority = Pubkey::from_str(&authority)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    let signer = Keypair::from_bytes(signer_bytes)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    let program_signer = Keypair::from_bytes(program_signer_bytes)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    let tx_hash = builder
        .initialize(&authority, &signer, &program_signer)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

    Ok(tx_hash)
}

#[pyfunction]
fn deploy(
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
) -> PyResult<String> {
    let builder = transaction_builder::TransactionBuilder::new_local();
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

    let tx_hash = builder
        .deploy(
            &authority,
            &signer,
            &base_asset,
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

    Ok(tx_hash)
}

#[pymodule]
fn boring_vault_svm(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(initialize, m)?)?;
    m.add_function(wrap_pyfunction!(deploy, m)?)?;
    Ok(())
}
