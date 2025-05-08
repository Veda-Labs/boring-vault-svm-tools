use std::collections::{hash_map::Entry, HashMap};
use std::path::PathBuf;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use eyre::Result;
use solana_client::rpc_client::RpcClient;
use solana_instruction::Instruction;
use solana_keypair::Keypair;
use solana_message::{Message, VersionedMessage};
use solana_pubkey::Pubkey;
use solana_sdk::transaction::VersionedTransaction;
use solana_signer::Signer;

use crate::{
    config::{KaminoConfig, VaultConfig},
    KeypairOrPublickey,
};

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

// The actual Builder implementation
pub struct Builder {
    pub client: RpcClient,
    pub instructions: Vec<Instruction>,
    pub signers: HashMap<Pubkey, Keypair>,
    pub kamino_config: KaminoConfig,
    pub vault_config: VaultConfig,
}

impl Default for Builder {
    fn default() -> Self {
        BuilderConfig::default().build()
    }
}

impl Builder {
    // TODO: remove
    pub fn new(rpc_url: String, path_root: Option<String>) -> Self {
        let mut config = BuilderConfig::default().with_rpc_url(rpc_url);

        if let Some(path) = path_root {
            config = config.with_data_path(path);
        }

        config.build()
    }

    pub fn clear(&mut self) -> Result<()> {
        self.instructions.clear();
        self.signers.clear();

        Ok(())
    }

    pub fn try_bundle_all(&mut self, payer: Keypair) -> Result<String> {
        let payer_pubkey = payer.pubkey();
        if !self.signers.contains_key(&payer.pubkey()) {
            self.signers.insert(payer_pubkey, payer);
        }

        let b64_tx = self.compile_to_versioned_transaction_b64(payer_pubkey)?;
        let serialized_tx = STANDARD.decode(&b64_tx)?;
        let versioned_tx: VersionedTransaction = bincode::deserialize(&serialized_tx)?;

        let result = self.client.send_and_confirm_transaction(&versioned_tx)?;

        self.instructions.clear();
        self.signers.clear();

        Ok(result.to_string())
    }

    pub fn compile_to_versioned_transaction_b64(&self, payer_pubkey: Pubkey) -> Result<String> {
        let blockhash = self.client.get_latest_blockhash()?;
        let message = VersionedMessage::Legacy(Message::new_with_blockhash(
            &self.instructions,
            Some(&payer_pubkey),
            &blockhash,
        ));

        let signers: Vec<&Keypair> = self.signers.values().collect();

        let tx = VersionedTransaction::try_new(message, &signers)?;
        let serialized_tx = bincode::serialize(&tx)?;

        Ok(STANDARD.encode(&serialized_tx))
    }

    pub fn add_signer_if_keypair(&mut self, potential_signer: KeypairOrPublickey) {
        if potential_signer.can_sign() {
            let pubkey = potential_signer.pubkey();
            if let Entry::Vacant(e) = self.signers.entry(pubkey) {
                if let Some(keypair) = potential_signer.into_keypair() {
                    e.insert(keypair);
                }
            }
        }
    }
}
