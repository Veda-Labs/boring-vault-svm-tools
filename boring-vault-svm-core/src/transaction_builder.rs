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

    pub fn initialize(
        &self,
        authority: &Pubkey,
        signer: &Keypair,
        program_signer: &Keypair,
    ) -> Result<String> {
        let ix = create_initialize_instruction(authority, &signer.pubkey())?;

        // Create the transaction
        let transaction = Transaction::new_signed_with_payer(
            &[ix],
            Some(&signer.pubkey()),
            &[signer, &program_signer],
            self.client.get_latest_blockhash()?,
        );

        let result = self.client.send_and_confirm_transaction(&transaction)?;

        Ok(result.to_string())
    }

    pub fn deploy(
        &self,
        authority: &Pubkey,
        signer: &Keypair,
        base_asset: &Pubkey,
        name: String,
        symbol: String,
        exchange_rate_provider: Option<Pubkey>,
        exchange_rate: u64,
        payout_address: Option<Pubkey>,
        allowed_exchange_rate_change_upper_bound: u16,
        allowed_exchange_rate_change_lower_bound: u16,
        minimum_update_delay_in_seconds: u32,
        platform_fee_bps: Option<u16>,
        performance_fee_bps: Option<u16>,
        withdraw_authority: Option<Pubkey>,
        strategist: Option<Pubkey>,
    ) -> Result<String> {
        let ix = create_deploy_instruction(
            &self.client,
            authority,
            &signer.pubkey(),
            base_asset,
            name,
            symbol,
            exchange_rate_provider,
            exchange_rate,
            payout_address,
            allowed_exchange_rate_change_upper_bound,
            allowed_exchange_rate_change_lower_bound,
            minimum_update_delay_in_seconds,
            platform_fee_bps,
            performance_fee_bps,
            withdraw_authority,
            strategist,
        )?;
        // Create the transaction
        let transaction = Transaction::new_signed_with_payer(
            &[ix],
            Some(&signer.pubkey()),
            &[signer],
            self.client.get_latest_blockhash()?,
        );

        let result = self.client.send_and_confirm_transaction(&transaction)?;

        Ok(result.to_string())
    }

    // TODO Add remaining calls
}
