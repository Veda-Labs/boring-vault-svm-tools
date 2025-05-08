#![allow(clippy::too_many_arguments)]
use boring_vault_svm_core::KeypairOrPublickey;
use pyo3::prelude::*;
use solana_keypair::Keypair;
use utils::to_keypair_from_bytes;

mod boring_vault;
mod kamino;
mod solend;
mod system;
mod utils;

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
}

#[pymodule]
fn boring_vault_svm(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Builder>()?;
    Ok(())
}
