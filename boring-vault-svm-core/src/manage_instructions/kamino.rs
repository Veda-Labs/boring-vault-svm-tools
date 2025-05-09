use std::str::FromStr;

use crate::impl_external_instruction_common;
use crate::manage_instructions::ExternalInstruction;
use crate::utils::bindings::boring_vault_svm::types::{Operator, Operators};
use crate::utils::{discriminator, pdas};

use solana_instruction::account_meta::AccountMeta;
use solana_program::{system_program, sysvar::instructions::ID as SYSVAR_INSTRUCTIONS_ID};
use solana_pubkey::{pubkey, Pubkey};
use spl_associated_token_account::get_associated_token_address_with_program_id;
use spl_token::ID as TOKEN_PROGRAM_ID;

pub const KAMINO_PROGRAM_ID: Pubkey = pubkey!("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD");
const KAMINO_FARMS_PROGRAM_ID: Pubkey = pubkey!("FarmsPZpWu9i7Kky8tPN37rs2TpmMrAZrC7S7vJa91Hr");
pub const KAMINO_SCOPE_PROGRAM_ID: Pubkey = pubkey!("HFn8GnPADiny6XqUoWE8uRPPxb29ikn4yTuPa9MF2fWJ");

pub struct KaminoInitUserMetaData {
    vault_id: u64,
    sub_account: u8,
    lut: Pubkey,
}

impl KaminoInitUserMetaData {
    pub fn new(vault_id: u64, sub_account: u8, lut: Pubkey) -> Self {
        Self {
            vault_id,
            sub_account,
            lut,
        }
    }
}

impl ExternalInstruction for KaminoInitUserMetaData {
    impl_external_instruction_common!();

    fn ix_program_id(&self) -> Pubkey {
        KAMINO_PROGRAM_ID
    }

    fn ix_data(&self) -> Vec<u8> {
        let mut ix_data =
            hex::decode("75a9b045c5170fa2").expect("Failed to decode hex discriminator"); // init_user_metadata instruction discriminator
        ix_data.extend_from_slice(&self.lut.to_bytes());
        ix_data
    }

    fn ix_remaining_accounts(&self) -> Vec<AccountMeta> {
        let from = pdas::get_vault_pda(self.vault_id, self.sub_account);
        let ix_remaining_accounts = vec![
            AccountMeta::new(from, false), // owner
            AccountMeta::new(from, false), // fee_payer
            AccountMeta::new(
                pdas::get_user_metadata_pda(&from, &self.ix_program_id()),
                false,
            ), // user_metadata
            AccountMeta::new_readonly(self.ix_program_id(), false), // referrer_user_metadata
            AccountMeta::new_readonly(solana_program::sysvar::rent::ID, false), // rent
            AccountMeta::new_readonly(system_program::ID, false), // system_program
        ];
        ix_remaining_accounts
    }

    fn ix_operators(&self) -> Operators {
        let operators = vec![
            Operator::IngestInstruction(0, 8),
            Operator::IngestInstructionDataSize,
        ];

        Operators { operators }
    }
}

pub struct KaminoInitObligation {
    vault_id: u64,
    sub_account: u8,
    lending_market: Pubkey,
    tag: u8,
    id: u8,
}

impl KaminoInitObligation {
    pub fn new(vault_id: u64, sub_account: u8, lending_market: Pubkey, tag: u8, id: u8) -> Self {
        Self {
            vault_id,
            sub_account,
            lending_market,
            tag,
            id,
        }
    }
}

impl ExternalInstruction for KaminoInitObligation {
    impl_external_instruction_common!();

    fn ix_program_id(&self) -> Pubkey {
        KAMINO_PROGRAM_ID
    }

    fn ix_data(&self) -> Vec<u8> {
        let mut ix_data =
            hex::decode("fb0ae74c1b0b9f60").expect("Failed to decode hex discriminator"); // init_obligation instruction discriminator
        ix_data.push(self.tag);
        ix_data.push(self.id);
        ix_data
    }

    fn ix_remaining_accounts(&self) -> Vec<AccountMeta> {
        let owner = pdas::get_vault_pda(self.vault_id, self.sub_account);
        let user_metadata = pdas::get_user_metadata_pda(&owner, &self.ix_program_id());
        let ix_remaining_accounts = vec![
            AccountMeta::new(owner, false), // obligation owner
            AccountMeta::new(owner, false), // fee_payer
            AccountMeta::new(
                pdas::get_obligation(
                    self.tag,
                    self.id,
                    &owner,
                    &self.lending_market,
                    &system_program::ID,
                    &system_program::ID,
                    &self.ix_program_id(),
                ),
                false,
            ), // obligation
            AccountMeta::new_readonly(self.lending_market, false), // lending market
            AccountMeta::new_readonly(system_program::ID, false), // seed_1
            AccountMeta::new_readonly(system_program::ID, false), // seed_2
            AccountMeta::new(user_metadata, false), // owner user metadata
            AccountMeta::new_readonly(solana_program::sysvar::rent::ID, false), // rent
            AccountMeta::new_readonly(system_program::ID, false), // system program
            AccountMeta::new_readonly(self.ix_program_id(), false), // ix program id
        ];
        ix_remaining_accounts
    }

    fn ix_operators(&self) -> Operators {
        let operators = vec![
            Operator::IngestInstruction(0, 8),
            Operator::IngestAccount(3),
            Operator::IngestInstructionDataSize,
        ];

        Operators { operators }
    }
}

pub struct KaminoInitObligationFarmsForReserve {
    vault_id: u64,
    sub_account: u8,
    reserve: Pubkey,
    reserve_farm_state: Pubkey,
    lending_market: Pubkey,
    tag: u8,
    id: u8,
    mode: u8,
}

impl KaminoInitObligationFarmsForReserve {
    pub fn new(
        vault_id: u64,
        sub_account: u8,
        reserve: Pubkey,
        reserve_farm_state: Pubkey,
        lending_market: Pubkey,
        tag: u8,
        id: u8,
        mode: u8,
    ) -> Self {
        Self {
            vault_id,
            sub_account,
            reserve,
            reserve_farm_state,
            lending_market,
            tag,
            id,
            mode,
        }
    }
}

impl ExternalInstruction for KaminoInitObligationFarmsForReserve {
    impl_external_instruction_common!();

    fn ix_program_id(&self) -> Pubkey {
        KAMINO_PROGRAM_ID
    }

    fn ix_data(&self) -> Vec<u8> {
        let mut ix_data =
            hex::decode("883f0fbad398a8a4").expect("Failed to decode hex discriminator");
        ix_data.push(self.mode);
        ix_data
    }

    fn ix_remaining_accounts(&self) -> Vec<AccountMeta> {
        let owner = pdas::get_vault_pda(self.vault_id, self.sub_account);
        let lending_market_authority =
            pdas::get_lending_market_authority(&self.lending_market, &self.ix_program_id());

        let obligation = pdas::get_obligation(
            self.tag,
            self.id,
            &owner,
            &self.lending_market,
            &system_program::ID,
            &system_program::ID,
            &self.ix_program_id(),
        );

        let obligation_farm = pdas::get_obligation_farm(
            &self.reserve_farm_state,
            &obligation,
            &KAMINO_FARMS_PROGRAM_ID,
        );

        vec![
            AccountMeta::new(owner, false),                    // obligation owner
            AccountMeta::new(owner, false),                    // fee_payer
            AccountMeta::new(obligation, false),               // obligation
            AccountMeta::new(lending_market_authority, false), // lending market authority
            AccountMeta::new(self.reserve, false),             // reserve
            AccountMeta::new(self.reserve_farm_state, false),  // reserve farm state
            AccountMeta::new(obligation_farm, false),          // obligation farm
            AccountMeta::new_readonly(self.lending_market, false), // lending market
            AccountMeta::new_readonly(KAMINO_FARMS_PROGRAM_ID, false), // farms program
            AccountMeta::new_readonly(solana_program::sysvar::rent::ID, false), // rent
            AccountMeta::new_readonly(system_program::ID, false), // system program
            AccountMeta::new_readonly(self.ix_program_id(), false), // ix program id
        ]
    }

    fn ix_operators(&self) -> Operators {
        let operators = vec![
            Operator::IngestInstruction(0, 8),
            Operator::IngestAccount(7),
            Operator::IngestInstructionDataSize,
        ];

        Operators { operators }
    }
}

pub struct KaminoRefreshReserve {
    vault_id: u64,
    sub_account: u8,
    reserve: Pubkey,
    lending_market: Pubkey,
    pyth_oracle: Pubkey,
    switchboard_price_oracle: Pubkey,
    switchboard_twap_oracle: Pubkey,
    scope_prices: Pubkey,
}

impl KaminoRefreshReserve {
    pub fn new(
        vault_id: u64,
        sub_account: u8,
        reserve: Pubkey,
        lending_market: Pubkey,
        pyth_oracle: Pubkey,
        switchboard_price_oracle: Pubkey,
        switchboard_twap_oracle: Pubkey,
        scope_prices: Pubkey,
    ) -> Self {
        Self {
            vault_id,
            sub_account,
            reserve,
            lending_market,
            pyth_oracle,
            switchboard_price_oracle,
            switchboard_twap_oracle,
            scope_prices,
        }
    }
}

impl ExternalInstruction for KaminoRefreshReserve {
    impl_external_instruction_common!();

    fn ix_program_id(&self) -> Pubkey {
        KAMINO_PROGRAM_ID
    }

    fn ix_data(&self) -> Vec<u8> {
        hex::decode("02da8aeb4fc91966").expect("Failed to decode hex discriminator")
    }

    fn ix_remaining_accounts(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(self.reserve, false), // reserve
            AccountMeta::new_readonly(self.lending_market, false), // lending market
            AccountMeta::new_readonly(self.pyth_oracle, false), // pyth oracle
            AccountMeta::new_readonly(self.switchboard_price_oracle, false), // switchboard price oracle
            AccountMeta::new_readonly(self.switchboard_twap_oracle, false), // switchboard twap oracle
            AccountMeta::new_readonly(self.scope_prices, false),            // scope prices
        ]
    }

    fn ix_operators(&self) -> Operators {
        let operators = vec![
            Operator::IngestInstruction(0, 8),
            Operator::IngestInstructionDataSize,
        ];

        Operators { operators }
    }
}

pub struct KaminoRefreshPriceList {
    vault_id: u64,
    sub_account: u8,
    oracle_prices: Pubkey,
    oracle_mapping: Pubkey,
    oracle_twaps: Pubkey,
    price_accounts: Vec<Pubkey>,
    tokens: Vec<u16>,
}

impl KaminoRefreshPriceList {
    pub fn new(
        vault_id: u64,
        sub_account: u8,
        oracle_prices: Pubkey,
        oracle_mapping: Pubkey,
        oracle_twaps: Pubkey,
        price_accounts: Vec<Pubkey>,
        tokens: Vec<u16>,
    ) -> Self {
        Self {
            vault_id,
            sub_account,
            oracle_prices,
            oracle_mapping,
            oracle_twaps,
            price_accounts,
            tokens,
        }
    }
}

impl ExternalInstruction for KaminoRefreshPriceList {
    impl_external_instruction_common!();

    fn ix_program_id(&self) -> Pubkey {
        KAMINO_SCOPE_PROGRAM_ID
    }

    fn ix_data(&self) -> Vec<u8> {
        let mut ix_data =
            hex::decode("53bacf83cbfec682").expect("Failed to decode hex discriminator");
        // First append the length of the vector as 4 bytes
        ix_data.extend_from_slice(&(self.tokens.len() as u32).to_le_bytes());

        // Then append each token as 2 bytes
        for token in &self.tokens {
            ix_data.extend_from_slice(&(*token).to_le_bytes());
        }

        ix_data
    }

    fn ix_remaining_accounts(&self) -> Vec<AccountMeta> {
        let mut ix_remaining_accounts = vec![
            AccountMeta::new(self.oracle_prices, false), // oracle prices
            AccountMeta::new_readonly(self.oracle_mapping, false), // oracle mappings
            AccountMeta::new(self.oracle_twaps, false),  // oracle twaps
            AccountMeta::new_readonly(SYSVAR_INSTRUCTIONS_ID, false), // sysvar instructions
        ];

        // Append each price account as readonly
        for price_account in &self.price_accounts {
            ix_remaining_accounts.push(AccountMeta::new_readonly(*price_account, false));
        }

        ix_remaining_accounts
    }

    fn ix_operators(&self) -> Operators {
        let operators = vec![
            Operator::IngestInstruction(0, 8),
            Operator::IngestInstructionDataSize,
        ];

        Operators { operators }
    }
}

pub struct KaminoRefreshObligation {
    vault_id: u64,
    sub_account: u8,
    lending_market: Pubkey,
    tag: u8,
    id: u8,
}

impl KaminoRefreshObligation {
    pub fn new(vault_id: u64, sub_account: u8, lending_market: Pubkey, tag: u8, id: u8) -> Self {
        Self {
            vault_id,
            sub_account,
            lending_market,
            tag,
            id,
        }
    }
}

impl ExternalInstruction for KaminoRefreshObligation {
    impl_external_instruction_common!();

    fn ix_program_id(&self) -> Pubkey {
        KAMINO_PROGRAM_ID
    }

    fn ix_data(&self) -> Vec<u8> {
        hex::decode("218493e497c04859").expect("Failed to decode hex discriminator")
    }

    fn ix_remaining_accounts(&self) -> Vec<AccountMeta> {
        let owner = pdas::get_vault_pda(self.vault_id, self.sub_account);

        let obligation = pdas::get_obligation(
            self.tag,
            self.id,
            &owner,
            &self.lending_market,
            &system_program::ID,
            &system_program::ID,
            &self.ix_program_id(),
        );
        vec![
            AccountMeta::new_readonly(self.lending_market, false),
            AccountMeta::new(obligation, false),
            AccountMeta::new(
                Pubkey::from_str("F9HdecRG8GPs9LEn4S5VfeJVEZVqrDJFR6bvmQTi22na").unwrap(),
                false,
            ), // farm accounts obligation farm user state
        ]
    }

    fn ix_operators(&self) -> Operators {
        let operators = vec![
            Operator::IngestInstruction(0, 8),
            Operator::IngestInstructionDataSize,
        ];

        Operators { operators }
    }
}

pub struct KaminoRefreshObligationFarmsForReserve {
    vault_id: u64,
    sub_account: u8,
    reserve: Pubkey,
    reserve_farm_state: Pubkey,
    lending_market: Pubkey,
    tag: u8,
    id: u8,
    mode: u8,
}

impl KaminoRefreshObligationFarmsForReserve {
    pub fn new(
        vault_id: u64,
        sub_account: u8,
        reserve: Pubkey,
        reserve_farm_state: Pubkey,
        lending_market: Pubkey,
        tag: u8,
        id: u8,
        mode: u8,
    ) -> Self {
        Self {
            vault_id,
            sub_account,
            reserve,
            reserve_farm_state,
            lending_market,
            tag,
            id,
            mode,
        }
    }
}

impl ExternalInstruction for KaminoRefreshObligationFarmsForReserve {
    impl_external_instruction_common!();

    fn ix_program_id(&self) -> Pubkey {
        KAMINO_PROGRAM_ID
    }

    fn ix_data(&self) -> Vec<u8> {
        let mut ix_data =
            hex::decode("8c90fd150a4af803").expect("Failed to decode hex discriminator");
        ix_data.push(self.mode);
        ix_data
    }

    fn ix_remaining_accounts(&self) -> Vec<AccountMeta> {
        let owner = pdas::get_vault_pda(self.vault_id, self.sub_account);
        let lending_market_authority =
            pdas::get_lending_market_authority(&self.lending_market, &self.ix_program_id());

        let obligation = pdas::get_obligation(
            self.tag,
            self.id,
            &owner,
            &self.lending_market,
            &system_program::ID,
            &system_program::ID,
            &self.ix_program_id(),
        );

        let obligation_farm = pdas::get_obligation_farm(
            &self.reserve_farm_state,
            &obligation,
            &KAMINO_FARMS_PROGRAM_ID,
        );

        vec![
            AccountMeta::new(owner, false),                            // crank
            AccountMeta::new(obligation, false),                       // obligation
            AccountMeta::new(lending_market_authority, false),         // lending market authority
            AccountMeta::new(self.reserve, false),                     // reserve
            AccountMeta::new(self.reserve_farm_state, false),          // reserve farm state
            AccountMeta::new(obligation_farm, false),                  // obligation farm
            AccountMeta::new_readonly(self.lending_market, false),     // lending market
            AccountMeta::new_readonly(KAMINO_FARMS_PROGRAM_ID, false), // farms program
            AccountMeta::new_readonly(solana_program::sysvar::rent::ID, false), // rent
            AccountMeta::new_readonly(system_program::ID, false),      // system program
        ]
    }

    fn ix_operators(&self) -> Operators {
        let operators = vec![
            Operator::IngestInstruction(0, 8),
            Operator::IngestAccount(6),
            Operator::IngestInstructionDataSize,
        ];

        Operators { operators }
    }
}

pub struct KaminoDeposit {
    vault_id: u64,
    sub_account: u8,
    lending_market: Pubkey,
    reserve: Pubkey,
    reserve_liquidity_mint: Pubkey,
    reserve_liquidity_supply: Pubkey,
    reserve_collateral_mint: Pubkey,
    reserve_destination_deposit_collateral: Pubkey,
    reserve_farm_state: Pubkey,
    tag: u8,
    id: u8,
    amount: u64,
}

impl KaminoDeposit {
    pub fn new(
        vault_id: u64,
        sub_account: u8,
        lending_market: Pubkey,
        reserve: Pubkey,
        reserve_liquidity_mint: Pubkey,
        reserve_liquidity_supply: Pubkey,
        reserve_collateral_mint: Pubkey,
        reserve_destination_deposit_collateral: Pubkey,
        reserve_farm_state: Pubkey,
        tag: u8,
        id: u8,
        amount: u64,
    ) -> Self {
        Self {
            vault_id,
            sub_account,
            lending_market,
            reserve,
            reserve_liquidity_mint,
            reserve_liquidity_supply,
            reserve_collateral_mint,
            reserve_destination_deposit_collateral,
            reserve_farm_state,
            tag,
            id,
            amount,
        }
    }
}

impl ExternalInstruction for KaminoDeposit {
    impl_external_instruction_common!();

    fn ix_program_id(&self) -> Pubkey {
        KAMINO_PROGRAM_ID
    }

    fn ix_data(&self) -> Vec<u8> {
        let discriminator = discriminator::get_anchor_discriminator(
            "deposit_reserve_liquidity_and_obligation_collateral_v2",
        );
        let mut ix_data = discriminator.to_vec();
        ix_data.extend_from_slice(&self.amount.to_le_bytes());
        ix_data
    }

    fn ix_remaining_accounts(&self) -> Vec<AccountMeta> {
        let owner = pdas::get_vault_pda(self.vault_id, self.sub_account);
        let lending_market_authority =
            pdas::get_lending_market_authority(&self.lending_market, &self.ix_program_id());
        let vault_mint_ata = get_associated_token_address_with_program_id(
            &owner,
            &self.reserve_liquidity_mint,
            &TOKEN_PROGRAM_ID,
        );
        let obligation = pdas::get_obligation(
            self.tag,
            self.id,
            &owner,
            &self.lending_market,
            &system_program::ID,
            &system_program::ID,
            &self.ix_program_id(),
        );

        let obligation_farm = pdas::get_obligation_farm(
            &self.reserve_farm_state,
            &obligation,
            &KAMINO_FARMS_PROGRAM_ID,
        );

        vec![
            AccountMeta::new(owner, false),                         // owner
            AccountMeta::new(obligation, false),                    // obligation
            AccountMeta::new_readonly(self.lending_market, false),  // lending market
            AccountMeta::new(lending_market_authority, false),      // lending market authority
            AccountMeta::new(self.reserve, false),                  // reserve
            AccountMeta::new(self.reserve_liquidity_mint, false), // reserve_destination_collateral
            AccountMeta::new(self.reserve_liquidity_supply, false), // reserve liquidity supply
            AccountMeta::new(self.reserve_collateral_mint, false), // reserve collateral mint
            AccountMeta::new(self.reserve_destination_deposit_collateral, false), // reserve destination deposit collateral
            AccountMeta::new(vault_mint_ata, false), // user source liquidity
            AccountMeta::new_readonly(self.ix_program_id(), false), // placeholder user destination collateral
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),     // collateral token program
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),     // liquidity token program
            AccountMeta::new_readonly(SYSVAR_INSTRUCTIONS_ID, false), // sysvar instruction
            AccountMeta::new(obligation_farm, false), // farm accounts obligation farm user state
            AccountMeta::new(self.reserve_farm_state, false), // farms accounts reserve farm state
            AccountMeta::new_readonly(KAMINO_FARMS_PROGRAM_ID, false), // farms program
        ]
    }

    fn ix_operators(&self) -> Operators {
        let operators = vec![
            Operator::IngestInstruction(0, 8),
            Operator::IngestAccount(2),
            Operator::IngestAccount(5),
            Operator::IngestInstructionDataSize,
        ];

        Operators { operators }
    }
}

pub struct KaminoBorrow {
    vault_id: u64,
    sub_account: u8,
    lending_market: Pubkey,
    reserve: Pubkey,
    borrow_reserve_liquidity_mint: Pubkey,
    reserve_source_liquidity: Pubkey,
    borrow_reserve_liquidity_fee_receiver: Pubkey,
    tag: u8,
    id: u8,
    amount: u64,
}

impl KaminoBorrow {
    pub fn new(
        vault_id: u64,
        sub_account: u8,
        lending_market: Pubkey,
        reserve: Pubkey,
        borrow_reserve_liquidity_mint: Pubkey,
        reserve_source_liquidity: Pubkey,
        borrow_reserve_liquidity_fee_receiver: Pubkey,
        tag: u8,
        id: u8,
        amount: u64,
    ) -> Self {
        Self {
            vault_id,
            sub_account,
            lending_market,
            reserve,
            borrow_reserve_liquidity_mint,
            reserve_source_liquidity,
            borrow_reserve_liquidity_fee_receiver,
            tag,
            id,
            amount,
        }
    }
}

impl ExternalInstruction for KaminoBorrow {
    impl_external_instruction_common!();

    fn ix_data(&self) -> Vec<u8> {
        let discriminator =
            hex::decode("a1808ff5abc7c206").expect("Failed to decode hex discriminator");
        let mut ix_data = discriminator.to_vec();
        ix_data.extend_from_slice(&self.amount.to_le_bytes());
        ix_data
    }

    fn ix_program_id(&self) -> Pubkey {
        KAMINO_PROGRAM_ID
    }

    fn ix_remaining_accounts(&self) -> Vec<AccountMeta> {
        let owner = pdas::get_vault_pda(self.vault_id, self.sub_account);
        let lending_market_authority =
            pdas::get_lending_market_authority(&self.lending_market, &self.ix_program_id());
        let vault_mint_ata = get_associated_token_address_with_program_id(
            &owner,
            &self.borrow_reserve_liquidity_mint,
            &TOKEN_PROGRAM_ID,
        );
        let obligation = pdas::get_obligation(
            self.tag,
            self.id,
            &owner,
            &self.lending_market,
            &system_program::ID,
            &system_program::ID,
            &self.ix_program_id(),
        );

        vec![
            AccountMeta::new(owner, false),                              // owner
            AccountMeta::new(obligation, false),                         // obligation
            AccountMeta::new_readonly(self.lending_market, false),       // lending market
            AccountMeta::new(lending_market_authority, false),           // lending market authority
            AccountMeta::new(self.reserve, false),                       // reserve
            AccountMeta::new(self.borrow_reserve_liquidity_mint, false), // borrow reserve liquidity mint
            AccountMeta::new(self.reserve_source_liquidity, false), // address = borrow_reserve.load()?.liquidity.supply_vault
            AccountMeta::new(self.borrow_reserve_liquidity_fee_receiver, false), // address = borrow_reserve.load()?.liquidity.fee_vault
            AccountMeta::new(vault_mint_ata, false), // user source liquidity
            AccountMeta::new(KAMINO_PROGRAM_ID, false), // refferer state
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),
            AccountMeta::new_readonly(SYSVAR_INSTRUCTIONS_ID, false),
            AccountMeta::new(KAMINO_PROGRAM_ID, false), // Farms Accounts Obligation Farm User State
            AccountMeta::new(KAMINO_PROGRAM_ID, false), // Farms Accounts Reserve Farm State
            AccountMeta::new_readonly(KAMINO_FARMS_PROGRAM_ID, false), // rent
        ]
    }

    fn ix_operators(&self) -> Operators {
        let operators = vec![
            Operator::IngestInstruction(0, 8),
            Operator::IngestAccount(2),
            Operator::IngestInstructionDataSize,
        ];

        Operators { operators }
    }
}
