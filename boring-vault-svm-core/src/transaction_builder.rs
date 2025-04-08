use crate::instructions::*;
use anchor_client::solana_sdk::signature::Keypair;
use eyre::Result;
use solana_client::rpc_client::RpcClient;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use solana_transaction::Transaction;

pub struct TransactionBuilder {
    client: RpcClient,
}

impl TransactionBuilder {
    pub fn new(rpc_url: String) -> Self {
        let client = RpcClient::new(rpc_url);

        Self { client }
    }

    pub fn new_local() -> Self {
        let client = RpcClient::new("http://127.0.0.1:8899".to_string());

        Self { client }
    }

    pub fn initialize(&self, authority: &Pubkey, signer: &Keypair) -> Result<()> {
        let ix = create_initialize_instruction(authority, &signer.pubkey())?;

        // Create the transaction
        let transaction = Transaction::new_signed_with_payer(
            &[ix],
            Some(&signer.pubkey()),
            &[signer],
            self.client.get_latest_blockhash()?,
        );

        self.client.send_and_confirm_transaction(&transaction)?;
        Ok(())
    }

    pub fn deploy(&self, authority: &Pubkey, signer: &Keypair) -> Result<()> {
        let ix = create_deploy_instruction(&self.client, authority, &signer.pubkey())?;
        // Create the transaction
        let transaction = Transaction::new_signed_with_payer(
            &[ix],
            Some(&signer.pubkey()),
            &[signer],
            self.client.get_latest_blockhash()?,
        );

        self.client.send_and_confirm_transaction(&transaction)?;

        Ok(())
    }

    // TODO Add remaining calls
}
