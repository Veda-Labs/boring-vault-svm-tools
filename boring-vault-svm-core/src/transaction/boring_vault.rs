use eyre::Result;
use solana_pubkey::Pubkey;

use crate::{
    instructions::{
        create_accept_authority_instruction, create_claim_fees_in_base_instruction,
        create_close_cpi_digest_instruction,
        create_configure_exchange_rate_update_bounds_instruction, create_deploy_instruction,
        create_deposit_instruction, create_deposit_sol_instruction, create_initialize_instruction,
        create_manage_instruction, create_pause_instruction,
        create_set_deposit_sub_account_instruction, create_set_fees_instruction,
        create_set_payout_instruction, create_set_strategist_instruction,
        create_set_withdraw_authority_instruction, create_set_withdraw_sub_account_instruction,
        create_transfer_authority_instruction, create_unpause_instruction,
        create_update_asset_data_instruction, create_update_exchange_rate_instruction,
        create_update_exchange_rate_provider_instruction, create_withdraw_instruction,
    },
    manage_instructions::TransferSolBetweenSubAccounts,
    KeypairOrPublickey,
};

use crate::builder::Builder;

impl Builder {
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
            &mint,
            allow_deposits,
            allow_withdrawals,
            share_premium_bps,
            is_pegged_to_base_asset,
            &price_feed,
            inverse_price_feed,
            max_staleness,
            min_samples,
        )?;

        // Add instruction
        self.instructions.push(ix);
        self.add_signer_if_keypair(signer);

        Ok(())
    }

    pub fn pause(&mut self, signer: KeypairOrPublickey, vault_id: u64) -> Result<()> {
        let ix = create_pause_instruction(vault_id, &signer.pubkey())?;

        self.instructions.push(ix);
        self.add_signer_if_keypair(signer);

        Ok(())
    }

    pub fn unpause(&mut self, signer: KeypairOrPublickey, vault_id: u64) -> Result<()> {
        let ix = create_unpause_instruction(vault_id, &signer.pubkey())?;

        self.instructions.push(ix);
        self.add_signer_if_keypair(signer);

        Ok(())
    }

    pub fn transfer_authority(
        &mut self,
        signer: KeypairOrPublickey,
        vault_id: u64,
        pending_authority: Pubkey,
    ) -> Result<()> {
        let ix =
            create_transfer_authority_instruction(vault_id, &signer.pubkey(), &pending_authority)?;

        self.instructions.push(ix);
        self.add_signer_if_keypair(signer);

        Ok(())
    }

    pub fn accept_authority(&mut self, signer: KeypairOrPublickey, vault_id: u64) -> Result<()> {
        let ix = create_accept_authority_instruction(vault_id, &signer.pubkey())?;

        self.instructions.push(ix);
        self.add_signer_if_keypair(signer);

        Ok(())
    }

    pub fn close_cpi_digest(
        &mut self,
        signer: KeypairOrPublickey,
        vault_id: u64,
        digest: [u8; 32],
    ) -> Result<()> {
        let ix = create_close_cpi_digest_instruction(vault_id, &signer.pubkey(), digest)?;

        self.instructions.push(ix);
        self.add_signer_if_keypair(signer);

        Ok(())
    }

    pub fn update_exchange_rate_provider(
        &mut self,
        signer: KeypairOrPublickey,
        vault_id: u64,
        new_provider: Pubkey,
    ) -> Result<()> {
        let ix = create_update_exchange_rate_provider_instruction(
            vault_id,
            &signer.pubkey(),
            &new_provider,
        )?;

        self.instructions.push(ix);
        self.add_signer_if_keypair(signer);

        Ok(())
    }

    pub fn set_withdraw_authority(
        &mut self,
        signer: KeypairOrPublickey,
        vault_id: u64,
        new_authority: Pubkey,
    ) -> Result<()> {
        let ix =
            create_set_withdraw_authority_instruction(vault_id, &signer.pubkey(), &new_authority)?;

        self.instructions.push(ix);
        self.add_signer_if_keypair(signer);

        Ok(())
    }

    pub fn set_payout(
        &mut self,
        signer: KeypairOrPublickey,
        vault_id: u64,
        new_payout: Pubkey,
    ) -> Result<()> {
        let ix = create_set_payout_instruction(vault_id, &signer.pubkey(), &new_payout)?;

        self.instructions.push(ix);
        self.add_signer_if_keypair(signer);

        Ok(())
    }

    pub fn configure_exchange_rate_update_bounds(
        &mut self,
        signer: KeypairOrPublickey,
        vault_id: u64,
        upper_bound: u16,
        lower_bound: u16,
        minimum_update_delay: u32,
    ) -> Result<()> {
        let ix = create_configure_exchange_rate_update_bounds_instruction(
            vault_id,
            &signer.pubkey(),
            upper_bound,
            lower_bound,
            minimum_update_delay,
        )?;

        self.instructions.push(ix);
        self.add_signer_if_keypair(signer);

        Ok(())
    }

    pub fn set_fees(
        &mut self,
        signer: KeypairOrPublickey,
        vault_id: u64,
        platform_fee_bps: u16,
        performance_fee_bps: u16,
    ) -> Result<()> {
        let ix = create_set_fees_instruction(
            vault_id,
            &signer.pubkey(),
            platform_fee_bps,
            performance_fee_bps,
        )?;

        self.instructions.push(ix);
        self.add_signer_if_keypair(signer);

        Ok(())
    }

    pub fn set_strategist(
        &mut self,
        signer: KeypairOrPublickey,
        vault_id: u64,
        new_strategist: Pubkey,
    ) -> Result<()> {
        let ix = create_set_strategist_instruction(vault_id, &signer.pubkey(), &new_strategist)?;

        self.instructions.push(ix);
        self.add_signer_if_keypair(signer);

        Ok(())
    }

    pub fn claim_fees_in_base(
        &mut self,
        signer: KeypairOrPublickey,
        vault_id: u64,
        sub_account: u8,
    ) -> Result<()> {
        let ix = create_claim_fees_in_base_instruction(
            &self.client,
            vault_id,
            sub_account,
            &signer.pubkey(),
        )?;

        self.instructions.extend(ix);
        self.add_signer_if_keypair(signer);

        Ok(())
    }

    pub fn deposit_sol(
        &mut self,
        signer: KeypairOrPublickey,
        vault_id: u64,
        deposit_amount: u64,
        min_mint_amount: u64,
    ) -> Result<()> {
        let ix = create_deposit_sol_instruction(
            &self.client,
            &signer.pubkey(),
            vault_id,
            deposit_amount,
            min_mint_amount,
        )?;

        // Add instruction
        self.instructions.extend(ix);
        self.add_signer_if_keypair(signer);

        Ok(())
    }

    // TODO: refresh boring vault asset data price feed
    pub fn deposit(
        &mut self,
        signer: KeypairOrPublickey,
        vault_id: u64,
        deposit_mint: Pubkey,
        deposit_amount: u64,
        min_mint_amount: u64,
    ) -> Result<()> {
        let ixs = create_deposit_instruction(
            &self.client,
            vault_id,
            &signer.pubkey(),
            &deposit_mint,
            deposit_amount,
            min_mint_amount,
        )?;

        self.instructions.extend(ixs);
        self.add_signer_if_keypair(signer);

        Ok(())
    }

    pub fn withdraw(
        &mut self,
        signer: KeypairOrPublickey,
        vault_id: u64,
        withdraw_mint: Pubkey,
        share_amount: u64,
        min_asset_amount: u64,
    ) -> Result<()> {
        let ix = create_withdraw_instruction(
            &self.client,
            vault_id,
            &signer.pubkey(),
            &withdraw_mint,
            share_amount,
            min_asset_amount,
        )?;

        self.instructions.extend(ix);
        self.add_signer_if_keypair(signer);

        Ok(())
    }

    pub fn update_exchange_rate(
        &mut self,
        signer: KeypairOrPublickey,
        vault_id: u64,
        new_exchange_rate: u64,
    ) -> Result<()> {
        let ix =
            create_update_exchange_rate_instruction(vault_id, &signer.pubkey(), new_exchange_rate)?;

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
}
