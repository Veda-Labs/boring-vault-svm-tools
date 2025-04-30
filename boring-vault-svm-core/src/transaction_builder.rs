use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::manage_instructions::{kamino::*, system::*};
use crate::utils::{get_lut_pda, get_vault_pda};
use crate::{instructions::*, KeypairOrPublickey};
use anchor_client::solana_sdk::signature::Keypair;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use bincode;
use eyre::Result;
use solana_client::rpc_client::RpcClient;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;
use solana_sdk::{
    message::{Message, VersionedMessage},
    transaction::VersionedTransaction,
};
use solana_signer::Signer;

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
        // 1. Add payer to signers if not present (needed for compile_... later)
        let payer_pubkey = payer.pubkey();
        if !self.signers.contains_key(&payer.pubkey()) {
            self.signers.insert(payer_pubkey, payer);
        }

        // 2. Compile the transaction using the new method
        let b64_tx = self.compile_to_versioned_transaction_b64(&payer_pubkey)?;

        // 3. Decode Base64
        let serialized_tx = STANDARD.decode(&b64_tx)?;

        // 4. Deserialize the transaction using bincode v1
        let mut versioned_tx: VersionedTransaction = bincode::deserialize(&serialized_tx)?;

        let recent_blockhash = self.client.get_latest_blockhash()?;
        versioned_tx.message.set_recent_blockhash(recent_blockhash);

        // 5. Send the deserialized VersionedTransaction
        let result = self.client.send_and_confirm_transaction(&versioned_tx)?;

        // 6. Clear builder state
        self.instructions.clear();
        self.signers.clear();

        // 7. Return transaction signature
        Ok(result.to_string())
    }

    /// Compiles the current instructions into a VersionedTransaction,
    /// signs it with all available keypairs in the builder, serializes it,
    /// and returns the Base64 encoded string.
    /// Does NOT send the transaction or clear the builder state.
    pub fn compile_to_versioned_transaction_b64(&self, payer_pubkey: &Pubkey) -> Result<String> {
        let message =
            VersionedMessage::Legacy(Message::new(&self.instructions, Some(payer_pubkey)));

        let signers: Vec<&Keypair> = self.signers.values().collect();

        let tx = VersionedTransaction::try_new(message, &signers)?;

        // Use bincode v1 API for serialization
        let serialized_tx = bincode::serialize(&tx)?;

        Ok(STANDARD.encode(&serialized_tx))
    }

    fn add_signer_if_keypair(&mut self, potential_signer: KeypairOrPublickey) {
        if potential_signer.can_sign() {
            let pubkey = potential_signer.pubkey();
            if let Entry::Vacant(e) = self.signers.entry(pubkey) {
                if let Some(keypair) = potential_signer.into_keypair() {
                    e.insert(keypair);
                }
            }
        }
    }

    pub fn initialize(
        &mut self,
        authority: Pubkey,
        signer: KeypairOrPublickey,
        program_signer: KeypairOrPublickey,
    ) -> Result<()> {
        let ix = create_initialize_instruction(&authority, &signer.pubkey())?;

        // Add instruction
        self.instructions.push(ix);

        self.add_signer_if_keypair(signer);
        self.add_signer_if_keypair(program_signer);

        Ok(())
    }

    pub fn deploy(
        &mut self,
        authority: Pubkey,
        signer: KeypairOrPublickey,
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

        self.instructions.push(ix);
        self.add_signer_if_keypair(signer);

        Ok(())
    }

    pub fn update_asset_data(
        &mut self,
        signer: KeypairOrPublickey,
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
        self.add_signer_if_keypair(signer);

        Ok(())
    }

    pub fn deposit_sol(
        &mut self,
        signer: KeypairOrPublickey,
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
        self.add_signer_if_keypair(signer);

        Ok(())
    }

    pub fn transfer_sol_between_sub_accounts(
        &mut self,
        signer: KeypairOrPublickey,
        authority: Option<KeypairOrPublickey>,
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

        self.add_signer_if_keypair(signer);
        if let Some(authority) = authority {
            self.add_signer_if_keypair(authority);
        }

        Ok(())
    }

    pub fn set_deposit_sub_account(
        &mut self,
        signer: KeypairOrPublickey,
        vault_id: u64,
        new_sub_account: u8,
    ) -> Result<()> {
        let ix = create_set_deposit_sub_account_instruction(
            &signer.pubkey(),
            vault_id,
            new_sub_account,
        )?;

        self.instructions.push(ix);
        self.add_signer_if_keypair(signer);

        Ok(())
    }

    pub fn set_withdraw_sub_account(
        &mut self,
        signer: KeypairOrPublickey,
        vault_id: u64,
        new_sub_account: u8,
    ) -> Result<()> {
        let ix = create_set_withdraw_sub_account_instruction(
            &signer.pubkey(),
            vault_id,
            new_sub_account,
        )?;

        self.instructions.push(ix);
        self.add_signer_if_keypair(signer);

        Ok(())
    }

    // TODO all these kamino calls should be bundled into a single tx and abstracted to one python function

    pub fn init_user_metadata(
        &mut self,
        signer: KeypairOrPublickey,
        authority: Option<KeypairOrPublickey>,
        vault_id: u64,
        sub_account: u8,
    ) -> Result<()> {
        // Add create lut instruction.
        let vault_pda = get_vault_pda(vault_id, sub_account);
        let recent_slot = self.client.get_slot()?;
        let create_lut_ix = create_lut_instruction(&signer.pubkey(), &vault_pda, recent_slot)?;

        self.instructions.push(create_lut_ix);

        let lut_account = get_lut_pda(&vault_pda, recent_slot);

        let eix = KaminoInitUserMetaData::new(vault_id, sub_account, lut_account);

        let ixs = match authority.as_ref() {
            Some(authority) => {
                create_manage_instruction(&self.client, &signer, Some(authority), eix)?
            }
            None => create_manage_instruction(&self.client, &signer, None, eix)?,
        };

        for ix in ixs {
            self.instructions.push(ix);
        }

        self.add_signer_if_keypair(signer);
        if let Some(authority) = authority {
            self.add_signer_if_keypair(authority);
        }

        Ok(())
    }

    pub fn init_obligation(
        &mut self,
        signer: KeypairOrPublickey,
        authority: Option<KeypairOrPublickey>,
        vault_id: u64,
        sub_account: u8,
        user_metadata: Pubkey,
        lending_market: Pubkey,
        tag: u8,
        id: u8,
    ) -> Result<()> {
        let eix = KaminoInitObligation::new(
            vault_id,
            sub_account,
            user_metadata,
            lending_market,
            tag,
            id,
        );

        let ixs = match authority.as_ref() {
            Some(authority) => {
                create_manage_instruction(&self.client, &signer, Some(authority), eix)?
            }
            None => create_manage_instruction(&self.client, &signer, None, eix)?,
        };

        for ix in ixs {
            self.instructions.push(ix);
        }

        self.add_signer_if_keypair(signer);
        if let Some(authority) = authority {
            self.add_signer_if_keypair(authority);
        }

        Ok(())
    }

    pub fn init_obligation_farms_for_reserve(
        &mut self,
        signer: KeypairOrPublickey,
        authority: Option<KeypairOrPublickey>,
        vault_id: u64,
        sub_account: u8,
        obligation: Pubkey,
        reserve: Pubkey,
        reserve_farm_state: Pubkey,
        obligation_farm: Pubkey,
        lending_market: Pubkey,
        farms_program: Pubkey,
        mode: u8,
    ) -> Result<()> {
        let eix = KaminoInitObligationFarmsForReserve::new(
            vault_id,
            sub_account,
            obligation,
            reserve,
            reserve_farm_state,
            obligation_farm,
            lending_market,
            farms_program,
            mode,
        );

        let ixs = match authority.as_ref() {
            Some(authority) => {
                create_manage_instruction(&self.client, &signer, Some(authority), eix)?
            }
            None => create_manage_instruction(&self.client, &signer, None, eix)?,
        };

        for ix in ixs {
            self.instructions.push(ix);
        }

        self.add_signer_if_keypair(signer);
        if let Some(authority) = authority {
            self.add_signer_if_keypair(authority);
        }

        Ok(())
    }

    pub fn refresh_reserve(
        &mut self,
        signer: KeypairOrPublickey,
        authority: Option<KeypairOrPublickey>,
        vault_id: u64,
        sub_account: u8,
        reserve: Pubkey,
        lending_market: Pubkey,
        pyth_oracle: Pubkey,
        switchboard_price_oracle: Pubkey,
        switchboard_twap_oracle: Pubkey,
        scope_prices: Pubkey,
    ) -> Result<()> {
        let eix = KaminoRefreshReserve::new(
            vault_id,
            sub_account,
            reserve,
            lending_market,
            pyth_oracle,
            switchboard_price_oracle,
            switchboard_twap_oracle,
            scope_prices,
        );

        let ixs = match authority.as_ref() {
            Some(authority) => {
                create_manage_instruction(&self.client, &signer, Some(authority), eix)?
            }
            None => create_manage_instruction(&self.client, &signer, None, eix)?,
        };

        for ix in ixs {
            self.instructions.push(ix);
        }

        self.add_signer_if_keypair(signer);
        if let Some(authority) = authority {
            self.add_signer_if_keypair(authority);
        }

        Ok(())
    }

    pub fn refresh_obligation(
        &mut self,
        signer: KeypairOrPublickey,
        authority: Option<KeypairOrPublickey>,
        vault_id: u64,
        sub_account: u8,
        lending_market: Pubkey,
        obligation: Pubkey,
    ) -> Result<()> {
        let eix = KaminoRefreshObligation::new(vault_id, sub_account, lending_market, obligation);

        let ixs = match authority.as_ref() {
            Some(authority) => {
                create_manage_instruction(&self.client, &signer, Some(authority), eix)?
            }
            None => create_manage_instruction(&self.client, &signer, None, eix)?,
        };

        for ix in ixs {
            self.instructions.push(ix);
        }

        self.add_signer_if_keypair(signer);
        if let Some(authority) = authority {
            self.add_signer_if_keypair(authority);
        }

        Ok(())
    }

    pub fn refresh_obligation_farms_for_reserve(
        &mut self,
        signer: KeypairOrPublickey,
        authority: Option<KeypairOrPublickey>,
        vault_id: u64,
        sub_account: u8,
        obligation: Pubkey,
        reserve: Pubkey,
        reserve_farm_state: Pubkey,
        obligation_farm: Pubkey,
        lending_market: Pubkey,
        farms_program: Pubkey,
        mode: u8,
    ) -> Result<()> {
        let eix = KaminoRefreshObligationFarmsForReserve::new(
            vault_id,
            sub_account,
            obligation,
            reserve,
            reserve_farm_state,
            obligation_farm,
            lending_market,
            farms_program,
            mode,
        );

        let ixs = match authority.as_ref() {
            Some(authority) => {
                create_manage_instruction(&self.client, &signer, Some(authority), eix)?
            }
            None => create_manage_instruction(&self.client, &signer, None, eix)?,
        };

        for ix in ixs {
            self.instructions.push(ix);
        }

        self.add_signer_if_keypair(signer);
        if let Some(authority) = authority {
            self.add_signer_if_keypair(authority);
        }

        Ok(())
    }

    pub fn refresh_price_list(
        &mut self,
        signer: KeypairOrPublickey,
        authority: Option<KeypairOrPublickey>,
        vault_id: u64,
        sub_account: u8,
        oracle_prices: Pubkey,
        oracle_mapping: Pubkey,
        oracle_twaps: Pubkey,
        price_accounts: Vec<Pubkey>,
        tokens: Vec<u16>,
    ) -> Result<()> {
        let eix = KaminoRefreshPriceList::new(
            vault_id,
            sub_account,
            oracle_prices,
            oracle_mapping,
            oracle_twaps,
            price_accounts,
            tokens,
        );

        let ixs = match authority.as_ref() {
            Some(authority) => {
                create_manage_instruction(&self.client, &signer, Some(authority), eix)?
            }
            None => create_manage_instruction(&self.client, &signer, None, eix)?,
        };

        for ix in ixs {
            self.instructions.push(ix);
        }

        self.add_signer_if_keypair(signer);
        if let Some(authority) = authority {
            self.add_signer_if_keypair(authority);
        }

        Ok(())
    }

    pub fn kamino_deposit(
        &mut self,
        signer: KeypairOrPublickey,
        authority: Option<KeypairOrPublickey>,
        vault_id: u64,
        sub_account: u8,
        lending_market: Pubkey,
        obligation: Pubkey,
        reserve: Pubkey,
        reserve_liquidity_mint: Pubkey,
        reserve_liquidity_supply: Pubkey,
        reserve_collateral_mint: Pubkey,
        reserve_destination_deposit_collateral: Pubkey,
        amount: u64,
    ) -> Result<()> {
        let eix = KaminoDeposit::new(
            vault_id,
            sub_account,
            lending_market,
            obligation,
            reserve,
            reserve_liquidity_mint,
            reserve_liquidity_supply,
            reserve_collateral_mint,
            reserve_destination_deposit_collateral,
            amount,
        );

        let ixs = match authority.as_ref() {
            Some(authority) => {
                create_manage_instruction(&self.client, &signer, Some(authority), eix)?
            }
            None => create_manage_instruction(&self.client, &signer, None, eix)?,
        };

        for ix in ixs {
            self.instructions.push(ix);
        }

        self.add_signer_if_keypair(signer);
        if let Some(authority) = authority {
            self.add_signer_if_keypair(authority);
        }

        Ok(())
    }

    pub fn deposit_solend(
        &mut self,
        signer: KeypairOrPublickey,
        authority: Option<KeypairOrPublickey>,
        vault_id: u64,
        sub_account: u8,
        deposit_mint: Pubkey,
        reserve_collateral_mint: Pubkey,
        lending_market: Pubkey,
        reserve: Pubkey,
        reserve_liquidity_supply_spl_token_account: Pubkey,
        lending_market_authority: Pubkey,
        destination_deposit_reserve_collateral_supply_spl_token_account: Pubkey,
        pyth_price_oracle: Pubkey,
        switchboard_price_oracle: Pubkey,
        amount: u64,
    ) -> Result<()> {
        let ixs = create_deposit_solend_instructions(
            &self.client,
            &signer,
            authority.as_ref(),
            vault_id,
            sub_account,
            &deposit_mint,
            &reserve_collateral_mint,
            &lending_market,
            &reserve,
            &reserve_liquidity_supply_spl_token_account,
            &lending_market_authority,
            &destination_deposit_reserve_collateral_supply_spl_token_account,
            &pyth_price_oracle,
            &switchboard_price_oracle,
            amount,
        )?;

        // Add all instructions
        for ix in ixs {
            self.instructions.push(ix);
        }

        self.add_signer_if_keypair(signer);
        if let Some(authority) = authority {
            self.add_signer_if_keypair(authority);
        }

        Ok(())
    }

    pub fn wrap_sol(
        &mut self,
        signer: KeypairOrPublickey,
        authority: Option<KeypairOrPublickey>,
        vault_id: u64,
        sub_account: u8,
        amount: u64,
    ) -> Result<()> {
        let ixs = create_wrap_sol_instructions(
            &self.client,
            &signer,
            authority.as_ref(),
            vault_id,
            sub_account,
            amount,
        )?;

        for ix in ixs {
            self.instructions.push(ix);
        }

        self.add_signer_if_keypair(signer);
        if let Some(authority) = authority {
            self.add_signer_if_keypair(authority);
        }

        Ok(())
    }

    pub fn unwrap_sol(
        &mut self,
        signer: KeypairOrPublickey,
        authority: Option<KeypairOrPublickey>,
        vault_id: u64,
        sub_account: u8,
    ) -> Result<()> {
        let ixs = create_unwrap_sol_instructions(
            &self.client,
            &signer,
            authority.as_ref(),
            vault_id,
            sub_account,
        )?;

        for ix in ixs {
            self.instructions.push(ix);
        }

        self.add_signer_if_keypair(signer);
        if let Some(authority) = authority {
            self.add_signer_if_keypair(authority);
        }

        Ok(())
    }

    pub fn mint_jito_sol(
        &mut self,
        signer: KeypairOrPublickey,
        authority: Option<KeypairOrPublickey>,
        vault_id: u64,
        sub_account: u8,
        amount: u64,
    ) -> Result<()> {
        let ixs = create_mint_jito_sol_instructions(
            &self.client,
            &signer,
            authority.as_ref(),
            vault_id,
            sub_account,
            amount,
        )?;

        for ix in ixs {
            self.instructions.push(ix);
        }

        self.add_signer_if_keypair(signer);
        if let Some(authority) = authority {
            self.add_signer_if_keypair(authority);
        }

        Ok(())
    }

    // TODO Add remaining calls
}
