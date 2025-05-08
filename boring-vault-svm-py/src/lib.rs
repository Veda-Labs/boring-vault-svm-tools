#![allow(clippy::too_many_arguments)]
use boring_vault_svm_core::config::BuilderConfig;
use pyo3::prelude::*;
use solana_keypair::Keypair;

mod boring_vault;
mod jito;
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
    fn new(
        rpc_url: Option<String>,
        data_path: Option<String>,
        kamino_file: Option<String>,
        vault_file: Option<String>,
        lend_mint: Option<String>,
        borrow_mint: Option<String>,
    ) -> Self {
        let mut config = BuilderConfig::default();

        if let Some(url) = rpc_url {
            config = config.with_rpc_url(url);
        }

        if let Some(path) = data_path {
            config = config.with_data_path(path);
        }

        if let Some(file) = kamino_file {
            config = config.with_kamino_file(file);
        }

        if let Some(file) = vault_file {
            config = config.with_vault_file(file);
        }

        if let Some(profile) = lend_mint {
            config = config.with_lend_mint(profile);
        }

        if let Some(profile) = borrow_mint {
            config = config.with_borrow_mint(profile);
        }

        Self {
            inner: config.build(),
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
}

#[pymodule]
fn boring_vault_svm(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Builder>()?;
    Ok(())
}
