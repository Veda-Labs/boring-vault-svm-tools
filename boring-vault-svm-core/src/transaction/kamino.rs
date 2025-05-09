use crate::{
    instructions::{create_lut_instruction, create_manage_instruction},
    manage_instructions::{
        ExternalInstruction, KaminoBorrow, KaminoDeposit, KaminoInitObligation,
        KaminoInitObligationFarmsForReserve, KaminoInitUserMetaData, KaminoRefreshObligation,
        KaminoRefreshObligationFarmsForReserve, KaminoRefreshPriceList, KaminoRefreshReserve,
        KAMINO_PROGRAM_ID,
    },
    utils::{ensure_ata, get_lut_pda, get_vault_pda, pdas},
    KeypairOrPublickey,
};

use crate::builder::Builder;

use eyre::Result;
use solana_pubkey::Pubkey;

impl Builder {
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
        tag: u8,
        id: u8,
    ) -> Result<()> {
        let eix = KaminoInitObligation::new(
            vault_id,
            sub_account,
            self.kamino_config.lending_market,
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
        tag: Option<u8>,
        id: Option<u8>,
        mode: u8,
    ) -> Result<()> {
        let eix = KaminoInitObligationFarmsForReserve::new(
            vault_id,
            sub_account,
            self.kamino_config.lend.reserve,
            self.kamino_config.lend.reserve_farm_state,
            self.kamino_config.lending_market,
            tag.unwrap_or(0),
            id.unwrap_or(0),
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
        vault_id: u64,
        sub_account: u8,
        reserve: Pubkey,
    ) -> Result<()> {
        let ix = KaminoRefreshReserve::new(
            vault_id,
            sub_account,
            reserve,
            self.kamino_config.lending_market,
            KAMINO_PROGRAM_ID,
            KAMINO_PROGRAM_ID,
            KAMINO_PROGRAM_ID,
            self.kamino_config.oracle_prices,
        );

        self.instructions.push(ix.to_instruction());

        Ok(())
    }

    pub fn refresh_reserves(
        &mut self,
        signer: KeypairOrPublickey,
        authority: Option<KeypairOrPublickey>,
        vault_id: u64,
        sub_account: u8,
    ) -> Result<()> {
        self.refresh_reserve(vault_id, sub_account, self.kamino_config.lend.reserve)?;
        self.refresh_reserve(vault_id, sub_account, self.kamino_config.borrow.reserve)?;

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
        tag: u8,
        id: u8,
    ) -> Result<()> {
        let eix = KaminoRefreshObligation::new(
            vault_id,
            sub_account,
            self.kamino_config.lending_market,
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

    pub fn refresh_obligation_farms_for_reserve(
        &mut self,
        signer: KeypairOrPublickey,
        authority: Option<KeypairOrPublickey>,
        vault_id: u64,
        sub_account: u8,
        tag: u8,
        id: u8,
        mode: u8,
    ) -> Result<()> {
        let eix = KaminoRefreshObligationFarmsForReserve::new(
            vault_id,
            sub_account,
            self.kamino_config.lend.reserve,
            self.kamino_config.lend.reserve_farm_state,
            self.kamino_config.lending_market,
            tag,
            id,
            mode,
        );

        self.instructions.push(eix.to_instruction());

        self.add_signer_if_keypair(signer);
        if let Some(authority) = authority {
            self.add_signer_if_keypair(authority);
        }

        Ok(())
    }

    pub fn refresh_price_list(
        &mut self,
        signer: KeypairOrPublickey,
        vault_id: u64,
        sub_account: u8,
    ) -> Result<()> {
        let price_accounts = self.kamino_config.price_accounts.clone();
        let tokens = self.kamino_config.tokens.clone();

        let eix = KaminoRefreshPriceList::new(
            vault_id,
            sub_account,
            self.kamino_config.oracle_prices,
            self.kamino_config.oracle_mapping,
            self.kamino_config.oracle_twaps,
            price_accounts,
            tokens,
        );

        self.instructions.push(eix.to_instruction());

        self.add_signer_if_keypair(signer);
        Ok(())
    }

    pub fn refresh_kamino_accounts(
        &mut self,
        vault_id: u64,
        sub_account: u8,
        tag: u8,
        id: u8,
    ) -> Result<()> {
        let refresh_obligation = KaminoRefreshObligation::new(
            vault_id,
            sub_account,
            self.kamino_config.lending_market,
            tag,
            id,
        );

        let refresh_lend = KaminoRefreshReserve::new(
            vault_id,
            sub_account,
            self.kamino_config.lend.reserve,
            self.kamino_config.lending_market,
            KAMINO_PROGRAM_ID,
            KAMINO_PROGRAM_ID,
            KAMINO_PROGRAM_ID,
            self.kamino_config.oracle_prices,
        );

        let refresh_borrow = KaminoRefreshReserve::new(
            vault_id,
            sub_account,
            self.kamino_config.borrow.reserve,
            self.kamino_config.lending_market,
            KAMINO_PROGRAM_ID,
            KAMINO_PROGRAM_ID,
            KAMINO_PROGRAM_ID,
            self.kamino_config.oracle_prices,
        );

        self.instructions.push(refresh_lend.to_instruction());
        self.instructions.push(refresh_borrow.to_instruction());
        self.instructions.push(refresh_obligation.to_instruction());

        Ok(())
    }

    pub fn lend(
        &mut self,
        signer: KeypairOrPublickey,
        authority: Option<KeypairOrPublickey>,
        vault_id: u64,
        sub_account: u8,
        amount: u64,
        tag: u8,
        id: u8,
    ) -> Result<()> {
        self.refresh_kamino_accounts(vault_id, sub_account, tag, id)?;

        let lend_ix = KaminoDeposit::new(
            vault_id,
            sub_account,
            self.kamino_config.lending_market,
            self.kamino_config.lend.reserve,
            self.kamino_config.lend.reserve_liquidity_mint,
            self.kamino_config.lend.reserve_liquidity_supply,
            self.kamino_config.lend.reserve_collateral_mint,
            self.kamino_config
                .lend
                .reserve_destination_deposit_collateral,
            self.kamino_config.lend.reserve_farm_state,
            tag,
            id,
            amount,
        );

        let manage_lend_ix = match authority.as_ref() {
            Some(authority) => {
                create_manage_instruction(&self.client, &signer, Some(authority), lend_ix)?
            }
            None => create_manage_instruction(&self.client, &signer, None, lend_ix)?,
        };

        self.instructions.extend(manage_lend_ix);

        self.add_signer_if_keypair(signer);
        if let Some(authority) = authority {
            self.add_signer_if_keypair(authority);
        }
        Ok(())
    }

    pub fn borrow(
        &mut self,
        signer: KeypairOrPublickey,
        authority: Option<KeypairOrPublickey>,
        vault_id: u64,
        sub_account: u8,
        amount: u64,
        tag: u8,
        id: u8,
    ) -> Result<()> {
        self.refresh_kamino_accounts(vault_id, sub_account, tag, id)?;

        let (_, ata_ix) = ensure_ata(
            &self.client,
            &signer.pubkey(),
            &pdas::get_vault_pda(vault_id, sub_account),
            &self.kamino_config.borrow.reserve_source_liquidity_mint,
            &spl_token::ID,
        )?;

        if let Some(uix) = ata_ix {
            self.instructions.push(uix);
        }

        let eix = KaminoBorrow::new(
            vault_id,
            sub_account,
            self.kamino_config.lending_market,
            self.kamino_config.borrow.reserve,
            self.kamino_config.borrow.reserve_source_liquidity_mint,
            self.kamino_config.borrow.reserve_source_liquidity,
            self.kamino_config
                .borrow
                .reserve_source_liquidity_fee_receiver,
            tag,
            id,
            amount,
        );

        let manage_borrow_ix = match authority.as_ref() {
            Some(authority) => {
                create_manage_instruction(&self.client, &signer, Some(authority), eix)?
            }
            None => create_manage_instruction(&self.client, &signer, None, eix)?,
        };

        self.instructions.extend(manage_borrow_ix);

        Ok(())
    }
}
