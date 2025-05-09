use boring_vault_svm_core::KeypairOrPublickey;
use pyo3::{pymethods, PyErr, PyResult};

use crate::{utils::to_keypair_from_bytes, Builder};

#[pymethods]
impl Builder {
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
}
