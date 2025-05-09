use std::{collections::HashMap, path::PathBuf};

use solana_client::rpc_client::RpcClient;

use crate::builder::Builder;

use super::{KaminoConfig, VaultConfig};

const DEFAULT_RPC_URL: &str = "https://api.devnet.solana.com";
const DEFAULT_DATA_PATH: &str = "../data";
const DEFAULT_KAMINO_FILE: &str = "kamino.json";
const DEFAULT_VAULT_FILE: &str = "vaults.json";
const DEFAULT_LEND_MINT: &str = "jito";
const DEFAULT_BORROW_MINT: &str = "sol";

pub struct BuilderConfig {
    rpc_url: String,
    data_path: String,
    kamino_file: String,
    vault_file: String,
    lend_mint: String,
    borrow_mint: String,
}

impl Default for BuilderConfig {
    fn default() -> Self {
        Self {
            rpc_url: DEFAULT_RPC_URL.to_string(),
            data_path: DEFAULT_DATA_PATH.to_string(),
            kamino_file: DEFAULT_KAMINO_FILE.to_string(),
            vault_file: DEFAULT_VAULT_FILE.to_string(),
            lend_mint: DEFAULT_LEND_MINT.to_string(),
            borrow_mint: DEFAULT_BORROW_MINT.to_string(),
        }
    }
}

impl BuilderConfig {
    pub fn with_rpc_url(mut self, rpc_url: impl Into<String>) -> Self {
        self.rpc_url = rpc_url.into();
        self
    }

    pub fn with_data_path(mut self, data_path: impl Into<String>) -> Self {
        self.data_path = data_path.into();
        self
    }

    pub fn with_kamino_file(mut self, kamino_file: impl Into<String>) -> Self {
        self.kamino_file = kamino_file.into();
        self
    }

    pub fn with_vault_file(mut self, vault_file: impl Into<String>) -> Self {
        self.vault_file = vault_file.into();
        self
    }

    pub fn with_lend_mint(mut self, lend_mint: impl Into<String>) -> Self {
        self.lend_mint = lend_mint.into();
        self
    }

    pub fn with_borrow_mint(mut self, borrow_mint: impl Into<String>) -> Self {
        self.borrow_mint = borrow_mint.into();
        self
    }

    pub fn build(self) -> Builder {
        let client = RpcClient::new(self.rpc_url);
        let instructions = vec![];
        let signers = HashMap::new();

        // Construct file paths
        let kamino_path = PathBuf::from(&self.data_path).join(&self.kamino_file);
        let vault_path = PathBuf::from(&self.data_path).join(&self.vault_file);

        // Load configurations
        let kamino_config = KaminoConfig::new(
            kamino_path.to_str().expect("Invalid path"),
            &self.lend_mint,
            &self.borrow_mint,
        )
        .expect("Failed to load Kamino config");

        let vault_config = VaultConfig::new(vault_path.to_str().expect("Invalid path"))
            .expect("Failed to load Vault config");

        Builder {
            client,
            instructions,
            signers,
            kamino_config,
            vault_config,
        }
    }
}
