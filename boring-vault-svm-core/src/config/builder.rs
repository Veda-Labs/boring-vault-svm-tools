use std::{collections::HashMap, path::PathBuf};

use solana_client::rpc_client::RpcClient;

use crate::builder::Builder;

use super::{KaminoConfig, VaultConfig};

const DEFAULT_RPC_URL: &str = "https://api.devnet.solana.com";
const DEFAULT_DATA_PATH: &str = "../data";
const DEFAULT_KAMINO_FILE: &str = "kamino.json";
const DEFAULT_VAULT_FILE: &str = "vaults.json";
const DEFAULT_LEND_PROFILE: &str = "jito";
const DEFAULT_BORROW_PROFILE: &str = "sol";

pub struct BuilderConfig {
    rpc_url: String,
    data_path: String,
    kamino_file: String,
    vault_file: String,
    lend_profile: String,
    borrow_profile: String,
}

impl Default for BuilderConfig {
    fn default() -> Self {
        Self {
            rpc_url: DEFAULT_RPC_URL.to_string(),
            data_path: DEFAULT_DATA_PATH.to_string(),
            kamino_file: DEFAULT_KAMINO_FILE.to_string(),
            vault_file: DEFAULT_VAULT_FILE.to_string(),
            lend_profile: DEFAULT_LEND_PROFILE.to_string(),
            borrow_profile: DEFAULT_BORROW_PROFILE.to_string(),
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

    pub fn with_lend_profile(mut self, lend_profile: impl Into<String>) -> Self {
        self.lend_profile = lend_profile.into();
        self
    }

    pub fn with_borrow_profile(mut self, borrow_profile: impl Into<String>) -> Self {
        self.borrow_profile = borrow_profile.into();
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
            &self.lend_profile,
            &self.borrow_profile,
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
