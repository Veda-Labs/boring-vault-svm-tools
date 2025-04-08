use boring_vault_svm_core::transaction_builder;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use std::str::FromStr;

#[pyfunction]
fn initialize(authority: String, signer_bytes: &[u8]) -> PyResult<()> {
    let builder = transaction_builder::TransactionBuilder::new_local();
    let authority = Pubkey::from_str(&authority)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    let signer = Keypair::from_bytes(signer_bytes)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    builder
        .initialize(&authority, &signer)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

    Ok(())
}

#[pyfunction]
fn deploy(authority: String, signer_bytes: &[u8]) -> PyResult<()> {
    let builder = transaction_builder::TransactionBuilder::new_local();
    let authority = Pubkey::from_str(&authority)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    let signer = Keypair::from_bytes(signer_bytes)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    builder
        .deploy(&authority, &signer)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

    Ok(())
}

#[pymodule]
fn boring_vault_svm(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(initialize, m)?)?;
    m.add_function(wrap_pyfunction!(deploy, m)?)?;
    Ok(())
}
