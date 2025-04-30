use std::str::FromStr;

use pyo3::{exceptions::PyValueError, PyResult};
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;

pub fn to_pubkey_from_string(authority: String) -> PyResult<Pubkey> {
    Pubkey::from_str(&authority).map_err(|e| PyValueError::new_err(e.to_string()))
}

pub fn to_keypair_from_bytes(signer_bytes: &[u8]) -> PyResult<Keypair> {
    Keypair::from_bytes(signer_bytes).map_err(|e| PyValueError::new_err(e.to_string()))
}
