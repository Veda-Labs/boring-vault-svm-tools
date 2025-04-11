use std::collections::HashMap;

use crate::instructions::*;
use crate::manage_instructions::external_instructions::*;
use crate::utils::{get_lut_pda, get_vault_pda};
use anchor_client::solana_sdk::signature::Keypair;
use eyre::Result;
use solana_client::rpc_client::RpcClient;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_signer::Signer;
use solana_transaction::Transaction;

pub struct TransactionBuilder {
    client: RpcClient,
    instructions: Vec<Instruction>,
    signers: HashMap<Pubkey, Keypair>,
}

impl TransactionBuilder {
    pub fn new(rpc_url: String) -> Self {
        let client = RpcClient::new(rpc_url);

        let instructions = vec![];
        let signers = HashMap::new();
        Self {
            client,
            instructions,
            signers,
        }
    }

    pub fn new_local() -> Self {
        let client = RpcClient::new("http://127.0.0.1:8899".to_string());

        let instructions = vec![];
        let signers = HashMap::new();
        Self {
            client,
            instructions,
            signers,
        }
    }

    pub fn clear(&mut self) -> Result<()> {
        // Clear both collections
        self.instructions.clear();
        self.signers.clear();

        Ok(())
    }

    pub fn try_bundle_all(&mut self, payer: Keypair) -> Result<String> {
        // Add payer to signers if not present
        let payer_pubkey = payer.pubkey();
        if !self.signers.contains_key(&payer.pubkey()) {
            self.signers.insert(payer.pubkey(), payer);
        }

        // Convert HashMap values to Vec<&Keypair>
        let signers: Vec<&Keypair> = self.signers.values().collect();

        // Create the transaction
        let transaction = Transaction::new_signed_with_payer(
            &self.instructions,
            Some(&payer_pubkey),
            &signers,
            self.client.get_latest_blockhash()?,
        );

        let msg_serialized = transaction.message().serialize();
        let signatures = transaction.signatures.len();

        // println!("Message size: {}", msg_serialized.len());
        // println!("Signatures size: {}", signatures * 64);
        // println!("Total Size: {}", msg_serialized.len() + signatures * 64);
        let total_size = msg_serialized.len() + signatures * 64;
        if total_size > 1232 {
            println!("TX Might be too large...");
            println!("Size: {}, Max Size: {}", total_size, 1232);
        }
        // docs https://solana.com/vi/docs/core/transactions
        // Message size: 559 bytes
        // Header: 65 bytes (32 + 32 + 1)
        // Signatures: 128 bytes (2 * 64)
        // Account metadata: 8 bytes (8 accounts * 1 byte)
        // Total: 559 + 65 + 128 + 8 = 760 bytes
        // Max solana tx size is 1232
        // According to the docs it seems like num signatures * 64 + msesage size serialized msut be less than or equal to 1232

        let result = self.client.send_and_confirm_transaction(&transaction)?;

        // Clear both collections
        self.instructions.clear();
        self.signers.clear();

        Ok(result.to_string())
    }

    pub fn initialize(
        &mut self,
        authority: Pubkey,
        signer: Keypair,
        program_signer: Keypair,
    ) -> Result<()> {
        let ix = create_initialize_instruction(&authority, &signer.pubkey())?;

        // Add instruction
        self.instructions.push(ix);

        // Update signers
        if !self.signers.contains_key(&signer.pubkey()) {
            self.signers.insert(signer.pubkey(), signer);
        }
        if !self.signers.contains_key(&program_signer.pubkey()) {
            self.signers.insert(program_signer.pubkey(), program_signer);
        }

        Ok(())
    }

    pub fn deploy(
        &mut self,
        authority: Pubkey,
        signer: Keypair,
        base_asset: Pubkey,
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
    ) -> Result<()> {
        let ix = create_deploy_instruction(
            &self.client,
            &authority,
            &signer.pubkey(),
            &base_asset,
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

        // Add instruction
        self.instructions.push(ix);

        // Update signers
        if !self.signers.contains_key(&signer.pubkey()) {
            self.signers.insert(signer.pubkey(), signer);
        }

        Ok(())
    }

    pub fn update_asset_data(
        &mut self,
        signer: Keypair,
        vault_id: u64,
        mint: Pubkey,
        allow_deposits: bool,
        allow_withdrawals: bool,
        share_premium_bps: u16,
        is_pegged_to_base_asset: bool,
        price_feed: Pubkey,
        inverse_price_feed: bool,
        max_staleness: u64,
        min_samples: u32,
    ) -> Result<()> {
        let ix = create_update_asset_data_instruction(
            &signer.pubkey(),
            vault_id,
            mint,
            allow_deposits,
            allow_withdrawals,
            share_premium_bps,
            is_pegged_to_base_asset,
            price_feed,
            inverse_price_feed,
            max_staleness,
            min_samples,
        )?;

        // Add instruction
        self.instructions.push(ix);

        // Update signers
        if !self.signers.contains_key(&signer.pubkey()) {
            self.signers.insert(signer.pubkey(), signer);
        }

        Ok(())
    }

    pub fn deposit_sol(
        &mut self,
        signer: Keypair,
        vault_id: u64,
        user_pubkey: Pubkey,
        deposit_amount: u64,
        min_mint_amount: u64,
    ) -> Result<()> {
        let ix = create_deposit_sol_instruction(
            &signer.pubkey(),
            vault_id,
            user_pubkey,
            deposit_amount,
            min_mint_amount,
        )?;

        // Add instruction
        self.instructions.push(ix);

        // Update signers
        if !self.signers.contains_key(&signer.pubkey()) {
            self.signers.insert(signer.pubkey(), signer);
        }

        Ok(())
    }

    pub fn transfer_sol_between_sub_accounts(
        &mut self,
        signer: Keypair,
        authority: Option<Keypair>,
        vault_id: u64,
        sub_account: u8,
        to_sub_account: u8,
        amount: u64,
    ) -> Result<()> {
        let eix = TransferSolBetweenSubAccounts::new(vault_id, sub_account, to_sub_account, amount);

        let ixs = match authority.as_ref() {
            Some(authority) => {
                create_manage_instruction(&self.client, &signer, Some(authority), eix)?
            }
            None => create_manage_instruction(&self.client, &signer, None, eix)?,
        };

        for ix in ixs {
            self.instructions.push(ix);
        }

        // Update signers
        if !self.signers.contains_key(&signer.pubkey()) {
            self.signers.insert(signer.pubkey(), signer);
        }

        if let Some(authority) = authority {
            if !self.signers.contains_key(&authority.pubkey()) {
                self.signers.insert(authority.pubkey(), authority);
            }
        }
        Ok(())
    }

    pub fn init_user_metadata(
        &mut self,
        signer: Keypair,
        authority: Option<Keypair>,
        vault_id: u64,
        sub_account: u8,
    ) -> Result<()> {
        // Add create lut instruction.
        let vault_pda = get_vault_pda(vault_id, sub_account);
        let recent_slot = self.client.get_slot()?;
        println!("here 0");
        let create_lut_ix = create_lut_instruction(&signer.pubkey(), &vault_pda, recent_slot)?;

        self.instructions.push(create_lut_ix);

        let lut_account = get_lut_pda(&vault_pda, recent_slot);

        let eix = KaminoInitUserMetaData::new(vault_id, sub_account, lut_account);

        println!("here 1");
        let ixs = match authority.as_ref() {
            Some(authority) => {
                create_manage_instruction(&self.client, &signer, Some(authority), eix)?
            }
            None => create_manage_instruction(&self.client, &signer, None, eix)?,
        };
        println!("here 2");

        for ix in ixs {
            self.instructions.push(ix);
        }

        // Update signers
        if !self.signers.contains_key(&signer.pubkey()) {
            self.signers.insert(signer.pubkey(), signer);
        }

        if let Some(authority) = authority {
            if !self.signers.contains_key(&authority.pubkey()) {
                self.signers.insert(authority.pubkey(), authority);
            }
        }

        Ok(())
    }

    // TODO Add remaining calls
}
