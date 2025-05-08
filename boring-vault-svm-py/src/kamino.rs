use boring_vault_svm_core::KeypairOrPublickey;
use pyo3::{pymethods, PyErr, PyResult};

use crate::{utils::to_keypair_from_bytes, Builder};

#[pymethods]
impl Builder {
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

    // !--- READ FUNCTIONS ---!

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
}
