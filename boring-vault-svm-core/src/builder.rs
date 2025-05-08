use std::collections::{hash_map::Entry, HashMap};

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

pub struct Builder {
    pub client: RpcClient,
    pub instructions: Vec<Instruction>,
    pub signers: HashMap<Pubkey, Keypair>,
    pub kamino_config: KaminoConfig,
    pub vault_config: VaultConfig,
}

impl Builder {
    pub fn new(rpc_url: String, path_root: Option<String>) -> Self {
        let client = RpcClient::new(rpc_url);

        let instructions = vec![];
        let signers = HashMap::new();
        let path_root = path_root.unwrap_or_else(|| "../data".to_string());
        let kamino_config =
            KaminoConfig::new(format!("{}/kamino.json", path_root).as_str(), "jito", "sol")
                .expect("Failed to load Kamino config");

        let vault_config = VaultConfig::new(format!("{}/vaults.json", path_root).as_str())
            .expect("Failed to load Vault config");

        Self {
            client,
            instructions,
            signers,
            kamino_config,
            vault_config,
        }
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

        // 4. Deserialize the transaction using bincode v1
        let versioned_tx: VersionedTransaction = bincode::deserialize(&serialized_tx)?;

        let result = self.client.send_and_confirm_transaction(&versioned_tx)?;

        self.instructions.clear();
        self.signers.clear();

        Ok(result.to_string())
    }

    /// Compiles the current instructions into a VersionedTransaction,
    /// signs it with all available keypairs in the builder, serializes it,
    /// and returns the Base64 encoded string.
    /// Does NOT send the transaction or clear the builder state.
    pub fn compile_to_versioned_transaction_b64(&self, payer_pubkey: Pubkey) -> Result<String> {
        let blockhash = self.client.get_latest_blockhash()?;
        let message = VersionedMessage::Legacy(Message::new_with_blockhash(
            &self.instructions,
            Some(&payer_pubkey),
            &blockhash,
        ));

        let signers: Vec<&Keypair> = self.signers.values().collect();

        let tx = VersionedTransaction::try_new(message, &signers)?;

        // Use bincode v1 API for serialization
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
