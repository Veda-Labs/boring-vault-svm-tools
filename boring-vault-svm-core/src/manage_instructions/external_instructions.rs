use crate::utils::bindings::boring_vault_svm::types::{Operator, Operators};
use crate::utils::{discriminator, get_obligation, get_user_metadata_pda, pdas};
use solana_instruction::account_meta::AccountMeta;
use solana_program::system_instruction;
use solana_program::{system_program, sysvar::instructions::ID as SYSVAR_INSTRUCTIONS_ID};
use solana_pubkey::{pubkey, Pubkey};
use spl_associated_token_account::get_associated_token_address_with_program_id;
use spl_token::ID as TOKEN_PROGRAM_ID;

pub trait ExternalInstruction {
    fn vault_id(&self) -> u64;
    fn sub_account(&self) -> u8;
    fn ix_program_id(&self) -> Pubkey;
    fn ix_data(&self) -> Vec<u8>;
    fn ix_remaining_accounts(&self) -> Vec<AccountMeta>;
    fn ix_operators(&self) -> Operators;
}

pub struct TransferSol {
    vault_id: u64,
    sub_account: u8,
    to: Pubkey,
    amount: u64,
}

impl TransferSol {
    pub fn new(vault_id: u64, sub_account: u8, to: Pubkey, amount: u64) -> Self {
        Self {
            vault_id,
            sub_account,
            to,
            amount,
        }
    }
}

impl ExternalInstruction for TransferSol {
    fn vault_id(&self) -> u64 {
        self.vault_id
    }
    fn sub_account(&self) -> u8 {
        self.sub_account
    }
    fn ix_program_id(&self) -> Pubkey {
        system_program::ID
    }

    fn ix_data(&self) -> Vec<u8> {
        let mut ix_data = vec![0x02, 0x00, 0x00, 0x00]; // Transfer instruction discriminator
        ix_data.extend_from_slice(&self.amount.to_le_bytes()); // Write amount as little-endian
        ix_data
    }

    fn ix_remaining_accounts(&self) -> Vec<AccountMeta> {
        let from = pdas::get_vault_pda(self.vault_id, self.sub_account);
        let ix_remaining_accounts = vec![
            AccountMeta::new(from, false),
            AccountMeta::new(self.to, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ];
        ix_remaining_accounts
    }

    fn ix_operators(&self) -> Operators {
        let operators = vec![
            Operator::IngestInstruction(0, 4),
            Operator::IngestAccount(1),
            Operator::IngestInstructionDataSize,
        ];

        Operators { operators }
    }
}

pub struct TransferSolBetweenSubAccounts {
    vault_id: u64,
    sub_account: u8,
    to_sub_account: u8,
    amount: u64,
}

impl TransferSolBetweenSubAccounts {
    pub fn new(vault_id: u64, sub_account: u8, to_sub_account: u8, amount: u64) -> Self {
        Self {
            vault_id,
            sub_account,
            to_sub_account,
            amount,
        }
    }
}

impl ExternalInstruction for TransferSolBetweenSubAccounts {
    fn vault_id(&self) -> u64 {
        self.vault_id
    }
    fn sub_account(&self) -> u8 {
        self.sub_account
    }
    fn ix_program_id(&self) -> Pubkey {
        system_program::ID
    }

    fn ix_data(&self) -> Vec<u8> {
        let mut ix_data = vec![0x02, 0x00, 0x00, 0x00]; // Transfer instruction discriminator
        ix_data.extend_from_slice(&self.amount.to_le_bytes()); // Write amount as little-endian
        ix_data
    }

    fn ix_remaining_accounts(&self) -> Vec<AccountMeta> {
        let from = pdas::get_vault_pda(self.vault_id, self.sub_account);
        let to = pdas::get_vault_pda(self.vault_id, self.to_sub_account);
        let ix_remaining_accounts = vec![
            AccountMeta::new(from, false),
            AccountMeta::new(to, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ];
        ix_remaining_accounts
    }

    fn ix_operators(&self) -> Operators {
        let operators = vec![
            Operator::IngestInstruction(0, 4),
            Operator::IngestAccount(1),
            Operator::IngestInstructionDataSize,
        ];

        Operators { operators }
    }
}

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
    fn vault_id(&self) -> u64 {
        self.vault_id
    }
    fn sub_account(&self) -> u8 {
        self.sub_account
    }
    fn ix_program_id(&self) -> Pubkey {
        pubkey!("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD")
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
            AccountMeta::new(get_user_metadata_pda(&from, &self.ix_program_id()), false), // user_metadata
            AccountMeta::new_readonly(self.ix_program_id(), false), // referrer_user_metadata
            AccountMeta::new_readonly(solana_program::sysvar::rent::ID, false), // rent
            AccountMeta::new_readonly(system_program::ID, false),   // system_program
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
    user_metadata: Pubkey,
    lending_market: Pubkey,
    tag: u8,
    id: u8,
}

impl KaminoInitObligation {
    pub fn new(
        vault_id: u64,
        sub_account: u8,
        user_metadata: Pubkey,
        lending_market: Pubkey,
        tag: u8,
        id: u8,
    ) -> Self {
        Self {
            vault_id,
            sub_account,
            user_metadata,
            lending_market,
            tag,
            id,
        }
    }
}

impl ExternalInstruction for KaminoInitObligation {
    fn vault_id(&self) -> u64 {
        self.vault_id
    }
    fn sub_account(&self) -> u8 {
        self.sub_account
    }
    fn ix_program_id(&self) -> Pubkey {
        pubkey!("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD")
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
        let ix_remaining_accounts = vec![
            AccountMeta::new(owner, false), // obligation owner
            AccountMeta::new(owner, false), // fee_payer
            AccountMeta::new(
                get_obligation(
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
            AccountMeta::new(self.user_metadata, false), // owner user metadata
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
    obligation: Pubkey,
    reserve: Pubkey,
    reserve_farm_state: Pubkey,
    obligation_farm: Pubkey,
    lending_market: Pubkey,
    farms_program: Pubkey,
    mode: u8,
}

impl KaminoInitObligationFarmsForReserve {
    pub fn new(
        vault_id: u64,
        sub_account: u8,
        obligation: Pubkey,
        reserve: Pubkey,
        reserve_farm_state: Pubkey,
        obligation_farm: Pubkey,
        lending_market: Pubkey,
        farms_program: Pubkey,
        mode: u8,
    ) -> Self {
        Self {
            vault_id,
            sub_account,
            obligation,
            reserve,
            reserve_farm_state,
            obligation_farm,
            lending_market,
            farms_program,
            mode,
        }
    }
}

impl ExternalInstruction for KaminoInitObligationFarmsForReserve {
    fn vault_id(&self) -> u64 {
        self.vault_id
    }

    fn sub_account(&self) -> u8 {
        self.sub_account
    }

    fn ix_program_id(&self) -> Pubkey {
        pubkey!("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD")
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
        vec![
            AccountMeta::new(owner, false),                    // obligation owner
            AccountMeta::new(owner, false),                    // fee_payer
            AccountMeta::new(self.obligation, false),          // obligation
            AccountMeta::new(lending_market_authority, false), // lending market authority
            AccountMeta::new(self.reserve, false),             // reserve
            AccountMeta::new(self.reserve_farm_state, false),  // reserve farm state
            AccountMeta::new(self.obligation_farm, false),     // obligation farm
            AccountMeta::new_readonly(self.lending_market, false), // lending market
            AccountMeta::new_readonly(self.farms_program, false), // farms program
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
    fn vault_id(&self) -> u64 {
        self.vault_id
    }

    fn sub_account(&self) -> u8 {
        self.sub_account
    }

    fn ix_program_id(&self) -> Pubkey {
        pubkey!("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD")
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
    fn vault_id(&self) -> u64 {
        self.vault_id
    }

    fn sub_account(&self) -> u8 {
        self.sub_account
    }

    fn ix_program_id(&self) -> Pubkey {
        pubkey!("HFn8GnPADiny6XqUoWE8uRPPxb29ikn4yTuPa9MF2fWJ")
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
    obligation: Pubkey,
    // TODO might need to add an optional vec of reserve accounts
    // TODO I DO!!!!
}

impl KaminoRefreshObligation {
    pub fn new(vault_id: u64, sub_account: u8, lending_market: Pubkey, obligation: Pubkey) -> Self {
        Self {
            vault_id,
            sub_account,
            lending_market,
            obligation,
        }
    }
}

impl ExternalInstruction for KaminoRefreshObligation {
    fn vault_id(&self) -> u64 {
        self.vault_id
    }

    fn sub_account(&self) -> u8 {
        self.sub_account
    }

    fn ix_program_id(&self) -> Pubkey {
        pubkey!("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD")
    }

    fn ix_data(&self) -> Vec<u8> {
        hex::decode("218493e497c04859").expect("Failed to decode hex discriminator")
    }

    fn ix_remaining_accounts(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new_readonly(self.lending_market, false), // lending market
            AccountMeta::new(self.obligation, false),              // obligation
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
    obligation: Pubkey,
    reserve: Pubkey,
    reserve_farm_state: Pubkey,
    obligation_farm: Pubkey,
    lending_market: Pubkey,
    farms_program: Pubkey,
    mode: u8,
}

impl KaminoRefreshObligationFarmsForReserve {
    pub fn new(
        vault_id: u64,
        sub_account: u8,
        obligation: Pubkey,
        reserve: Pubkey,
        reserve_farm_state: Pubkey,
        obligation_farm: Pubkey,
        lending_market: Pubkey,
        farms_program: Pubkey,
        mode: u8,
    ) -> Self {
        Self {
            vault_id,
            sub_account,
            obligation,
            reserve,
            reserve_farm_state,
            obligation_farm,
            lending_market,
            farms_program,
            mode,
        }
    }
}

impl ExternalInstruction for KaminoRefreshObligationFarmsForReserve {
    fn vault_id(&self) -> u64 {
        self.vault_id
    }

    fn sub_account(&self) -> u8 {
        self.sub_account
    }

    fn ix_program_id(&self) -> Pubkey {
        pubkey!("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD")
    }

    fn ix_data(&self) -> Vec<u8> {
        let mut ix_data =
            hex::decode("8c90fd150a4af803").expect("Failed to decode hex discriminator");
        ix_data.push(self.mode);
        ix_data
    }

    fn ix_remaining_accounts(&self) -> Vec<AccountMeta> {
        let owner = pdas::get_vault_pda(self.vault_id, self.sub_account);
        println!("vault pda: {}", owner);
        let lending_market_authority =
            pdas::get_lending_market_authority(&self.lending_market, &self.ix_program_id());
        vec![
            AccountMeta::new(owner, false),                        // crank
            AccountMeta::new(self.obligation, false),              // obligation
            AccountMeta::new(lending_market_authority, false),     // lending market authority
            AccountMeta::new(self.reserve, false),                 // reserve
            AccountMeta::new(self.reserve_farm_state, false),      // reserve farm state
            AccountMeta::new(self.obligation_farm, false),         // obligation farm
            AccountMeta::new_readonly(self.lending_market, false), // lending market
            AccountMeta::new_readonly(self.farms_program, false),  // farms program
            AccountMeta::new_readonly(solana_program::sysvar::rent::ID, false), // rent
            AccountMeta::new_readonly(system_program::ID, false),  // system program
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
    obligation: Pubkey,
    lending_market: Pubkey,
    reserve: Pubkey,
    reserve_liquidity_mint: Pubkey,
    reserve_liquidity_supply: Pubkey,
    reserve_collateral_mint: Pubkey,
    reserve_destination_deposit_collateral: Pubkey,
    amount: u64,
}

impl KaminoDeposit {
    pub fn new(
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
    ) -> Self {
        Self {
            vault_id,
            sub_account,
            obligation,
            lending_market,
            reserve,
            reserve_liquidity_mint,
            reserve_liquidity_supply,
            reserve_collateral_mint,
            reserve_destination_deposit_collateral,
            amount,
        }
    }
}

impl ExternalInstruction for KaminoDeposit {
    fn vault_id(&self) -> u64 {
        self.vault_id
    }

    fn sub_account(&self) -> u8 {
        self.sub_account
    }

    fn ix_program_id(&self) -> Pubkey {
        pubkey!("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD")
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
        vec![
            AccountMeta::new(owner, false),                         // owner
            AccountMeta::new(self.obligation, false),               // obligation
            AccountMeta::new_readonly(self.lending_market, false),  // lending market
            AccountMeta::new(lending_market_authority, false),      // lending market authority
            AccountMeta::new(self.reserve, false),                  // reserve
            AccountMeta::new(self.reserve_liquidity_mint, false),   // reserve liquidity mint
            AccountMeta::new(self.reserve_liquidity_supply, false), // reserve liquidity supply
            AccountMeta::new(self.reserve_collateral_mint, false),  // reserve collateral mint
            AccountMeta::new(self.reserve_destination_deposit_collateral, false), // reserve destination deposit collateral
            AccountMeta::new(vault_mint_ata, false), // user source liquidity
            AccountMeta::new_readonly(self.ix_program_id(), false), // placeholder user destination collateral
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),     // collateral token program
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),     // liquidity token program
            AccountMeta::new_readonly(SYSVAR_INSTRUCTIONS_ID, false), // sysvar instruction
            AccountMeta::new(
                pubkey!("GZGqnppbrZeBwmW8413jtj7pPNtdJo8CmN69Ymq8Dg8t"),
                false,
            ), // farm accounts obligation farm user state
            AccountMeta::new(
                pubkey!("B4mX639wYzxmMVgPno2wZUEPjTdbDGs5VD7TG7FNmy7P"),
                false,
            ), // farms accounts reserve farm state
            AccountMeta::new_readonly(
                pubkey!("FarmsPZpWu9i7Kky8tPN37rs2TpmMrAZrC7S7vJa91Hr"),
                false,
            ), // farms program
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

pub struct SolendInitObligation {
    vault_id: u64,
    sub_account: u8,
    obligation: Pubkey,
    lending_market: Pubkey,
}

impl SolendInitObligation {
    pub fn new(vault_id: u64, sub_account: u8, obligation: Pubkey, lending_market: Pubkey) -> Self {
        Self {
            vault_id,
            sub_account,
            obligation,
            lending_market,
        }
    }
}

impl ExternalInstruction for SolendInitObligation {
    fn vault_id(&self) -> u64 {
        self.vault_id
    }

    fn sub_account(&self) -> u8 {
        self.sub_account
    }

    fn ix_program_id(&self) -> Pubkey {
        pubkey!("So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo")
    }

    fn ix_data(&self) -> Vec<u8> {
        hex::decode("06").expect("Failed to decode hex discriminator")
    }

    fn ix_remaining_accounts(&self) -> Vec<AccountMeta> {
        let owner = pdas::get_vault_pda(self.vault_id, self.sub_account);
        vec![
            AccountMeta::new(self.obligation, false), // obligation
            AccountMeta::new_readonly(self.lending_market, false), // lending market
            AccountMeta::new(owner, false),           // owner
            AccountMeta::new_readonly(solana_program::sysvar::rent::ID, false), // rent
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false), // token program
        ]
    }

    fn ix_operators(&self) -> Operators {
        let operators = vec![
            Operator::IngestInstruction(0, 1),
            Operator::IngestAccount(1),
            Operator::IngestInstructionDataSize,
        ];

        Operators { operators }
    }
}

pub struct SolendDepositReserveLiquidityAndObligationCollateral {
    vault_id: u64,
    sub_account: u8,
    deposit_mint: Pubkey,
    reserve_collateral_mint: Pubkey,
    reserve: Pubkey,
    reserve_liquidity_supply_spl_token_account: Pubkey,
    lending_market: Pubkey,
    lending_market_authority: Pubkey,
    destination_deposit_reserve_collateral_supply_spl_token_account: Pubkey,
    obligation: Pubkey,
    pyth_price_oracle: Pubkey,
    switchboard_price_oracle: Pubkey,
    amount: u64,
}

impl SolendDepositReserveLiquidityAndObligationCollateral {
    pub fn new(
        vault_id: u64,
        sub_account: u8,
        deposit_mint: Pubkey,
        reserve_collateral_mint: Pubkey,
        reserve: Pubkey,
        reserve_liquidity_supply_spl_token_account: Pubkey,
        lending_market: Pubkey,
        lending_market_authority: Pubkey,
        destination_deposit_reserve_collateral_supply_spl_token_account: Pubkey,
        obligation: Pubkey,
        pyth_price_oracle: Pubkey,
        switchboard_price_oracle: Pubkey,
        amount: u64,
    ) -> Self {
        Self {
            vault_id,
            sub_account,
            deposit_mint,
            reserve_collateral_mint,
            reserve,
            reserve_liquidity_supply_spl_token_account,
            lending_market,
            lending_market_authority,
            destination_deposit_reserve_collateral_supply_spl_token_account,
            obligation,
            pyth_price_oracle,
            switchboard_price_oracle,
            amount,
        }
    }
}

impl ExternalInstruction for SolendDepositReserveLiquidityAndObligationCollateral {
    fn vault_id(&self) -> u64 {
        self.vault_id
    }

    fn sub_account(&self) -> u8 {
        self.sub_account
    }

    fn ix_program_id(&self) -> Pubkey {
        pubkey!("So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo")
    }

    fn ix_data(&self) -> Vec<u8> {
        // 0e40420f0000000000
        let mut ix_data = hex::decode("0e").expect("Failed to decode hex discriminator");
        ix_data.extend_from_slice(&self.amount.to_le_bytes());
        ix_data
    }

    fn ix_remaining_accounts(&self) -> Vec<AccountMeta> {
        let owner = pdas::get_vault_pda(self.vault_id, self.sub_account);
        let vault_deposit_ata = get_associated_token_address_with_program_id(
            &owner,
            &self.deposit_mint,
            &TOKEN_PROGRAM_ID,
        );
        let vault_share_ata = get_associated_token_address_with_program_id(
            &owner,
            &self.reserve_collateral_mint,
            &TOKEN_PROGRAM_ID,
        );
        vec![
            AccountMeta::new(vault_deposit_ata, false), // where deposit comes from
            AccountMeta::new(vault_share_ata, false),   // where shares go
            AccountMeta::new(self.reserve, false),      // reserve
            AccountMeta::new(self.reserve_liquidity_supply_spl_token_account, false), // reserve_liquidity_supply_spl_token_account
            AccountMeta::new(self.reserve_collateral_mint, false), // reserve_collateral_mint
            AccountMeta::new(self.lending_market, false),          // lending market
            AccountMeta::new(self.lending_market_authority, false), // lending market authority
            AccountMeta::new(
                self.destination_deposit_reserve_collateral_supply_spl_token_account,
                false,
            ), // destination_deposit_reserve_collateral_supply_spl_token_account
            AccountMeta::new(self.obligation, false),              // obligation
            AccountMeta::new(owner, false),                        // owner
            AccountMeta::new_readonly(self.pyth_price_oracle, false), // pyth price oracle
            AccountMeta::new_readonly(self.switchboard_price_oracle, false), // switchboard price oracle
            AccountMeta::new(owner, false), // user transfer authority
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false), // token program
        ]
    }

    fn ix_operators(&self) -> Operators {
        let operators = vec![
            Operator::IngestInstruction(0, 1),
            Operator::IngestAccount(1),
            Operator::IngestInstructionDataSize,
        ];

        Operators { operators }
    }
}

pub struct CreateAccountWithSeed {
    vault_id: u64,
    sub_account: u8,
    seed_str: String,
    lamports: u64,
    space: u64,
    owner: Pubkey,
}

impl CreateAccountWithSeed {
    pub fn new(
        vault_id: u64,
        sub_account: u8,
        seed_str: String,
        lamports: u64,
        space: u64,
        owner: Pubkey,
    ) -> Self {
        Self {
            vault_id,
            sub_account,
            seed_str,
            lamports,
            space,
            owner,
        }
    }
}

impl ExternalInstruction for CreateAccountWithSeed {
    fn vault_id(&self) -> u64 {
        self.vault_id
    }

    fn sub_account(&self) -> u8 {
        self.sub_account
    }

    fn ix_program_id(&self) -> Pubkey {
        system_program::ID
    }

    fn ix_data(&self) -> Vec<u8> {
        let vault_pda = pdas::get_vault_pda(self.vault_id, self.sub_account);
        let pda = Pubkey::create_with_seed(&vault_pda, &self.seed_str, &self.owner)
            .expect("Failed to create account address with seed");

        // Use the system instruction helper to generate the data
        let ix = system_instruction::create_account_with_seed(
            &vault_pda,     // from (funding account)
            &pda,           // to (the account to create)
            &vault_pda,     // base
            &self.seed_str, // seed string
            self.lamports,  // amount of lamports
            self.space,     // amount of space in bytes
            &self.owner,    // owner of the created account
        );
        ix.data
    }

    fn ix_remaining_accounts(&self) -> Vec<AccountMeta> {
        let vault_pda = pdas::get_vault_pda(self.vault_id, self.sub_account);
        let pda = Pubkey::create_with_seed(&vault_pda, &self.seed_str, &self.owner)
            .expect("Failed to create account address with seed");

        vec![
            AccountMeta::new(vault_pda, false),
            AccountMeta::new(pda, false),
        ]
    }

    fn ix_operators(&self) -> Operators {
        let operators = vec![
            Operator::IngestInstruction(0, 4),
            Operator::IngestAccount(0),
            Operator::IngestInstructionDataSize,
        ];

        Operators { operators }
    }
}

// unwrapping
// call close account
pub struct CloseAccount {
    vault_id: u64,
    sub_account: u8,
    account: Pubkey,
    token_program: Pubkey,
}

impl CloseAccount {
    pub fn new(vault_id: u64, sub_account: u8, account: Pubkey, token_program: Pubkey) -> Self {
        Self {
            vault_id,
            sub_account,
            account,
            token_program,
        }
    }
}

impl ExternalInstruction for CloseAccount {
    fn vault_id(&self) -> u64 {
        self.vault_id
    }

    fn sub_account(&self) -> u8 {
        self.sub_account
    }

    fn ix_program_id(&self) -> Pubkey {
        self.token_program
    }

    fn ix_data(&self) -> Vec<u8> {
        vec![9] // 9 is the discriminator for closing an account.
    }

    fn ix_remaining_accounts(&self) -> Vec<AccountMeta> {
        let vault_pda = pdas::get_vault_pda(self.vault_id, self.sub_account);

        vec![
            AccountMeta::new(self.account, false),       // Account to close
            AccountMeta::new(vault_pda, false),          // destination
            AccountMeta::new_readonly(vault_pda, false), // owner
        ]
    }

    fn ix_operators(&self) -> Operators {
        let operators = vec![
            Operator::IngestInstruction(0, 1),
            Operator::IngestAccount(1),
            Operator::IngestInstructionDataSize,
        ];

        Operators { operators }
    }
}

// Minting JitoSOL
pub struct MintJitoSol {
    vault_id: u64,
    sub_account: u8,
    amount: u64,
}

impl MintJitoSol {
    pub fn new(vault_id: u64, sub_account: u8, amount: u64) -> Self {
        Self {
            vault_id,
            sub_account,
            amount,
        }
    }
}

impl ExternalInstruction for MintJitoSol {
    fn vault_id(&self) -> u64 {
        self.vault_id
    }

    fn sub_account(&self) -> u8 {
        self.sub_account
    }

    fn ix_program_id(&self) -> Pubkey {
        pubkey!("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy")
    }

    fn ix_data(&self) -> Vec<u8> {
        let mut ix_data = vec![14]; // 14 is the discriminator for minting JitoSol.
        ix_data.extend(self.amount.to_le_bytes());

        ix_data
    }

    fn ix_remaining_accounts(&self) -> Vec<AccountMeta> {
        let vault_pda = pdas::get_vault_pda(self.vault_id, self.sub_account);
        let mint = pubkey!("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn");
        let jito_sol_ata =
            get_associated_token_address_with_program_id(&vault_pda, &mint, &TOKEN_PROGRAM_ID);

        vec![
            AccountMeta::new(
                pubkey!("Jito4APyf642JPZPx3hGc6WWJ8zPKtRbRs4P815Awbb"),
                false,
            ), // Jito (JitoSol) Stake Pool
            AccountMeta::new_readonly(
                pubkey!("6iQKfEyhr3bZMotVkW6beNZz5CPAkiwvgV2CTje9pVSS"),
                false,
            ), // stake pool withdraw authority
            AccountMeta::new(
                pubkey!("BgKUXdS29YcHCFrPm5M8oLHiTzZaMDjsebggjoaQ6KFL"),
                false,
            ), // reserve stake account
            AccountMeta::new(vault_pda, false),    // depositor
            AccountMeta::new(jito_sol_ata, false), // user account
            AccountMeta::new(
                pubkey!("feeeFLLsam6xZJFc6UQFrHqkvVt4jfmVvi2BRLkUZ4i"),
                false,
            ), // fee account
            AccountMeta::new(jito_sol_ata, false), // referral fee account
            AccountMeta::new(mint, false),         // token mint
            AccountMeta::new_readonly(system_program::ID, false), // system program
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false), // token program
        ]
    }

    fn ix_operators(&self) -> Operators {
        let operators = vec![
            Operator::IngestInstruction(0, 1),
            Operator::IngestAccount(3),
            Operator::IngestAccount(4),
            Operator::IngestInstructionDataSize,
        ];

        Operators { operators }
    }
}

// call unstake on pool, then need to call deactivate, then finally withdraw after some days

// TODO Jupiter DEX plan
// Looks like an anchor program, so ingest discriminator
// Do not ingest ix data size as that will change betweeen calls
// Ingest destination token account, source mint, destination mint
// example txs
// https://solscan.io/tx/5LqV9oZUitaPieA3pth1JZoyo2XPcSfZawWioEtPUWAqTdRbDqqwy2Mjuw3cLM2mkcXeA4E5yAhFdhuYR9yYPtBz
// https://solscan.io/tx/27fTvBLo2GuBLLL1jSEGKLqKtcioFwQQG6mNw4vcF54G9iE8FLBU1wQZWNZGkeqRocnf772Qx1DWSkyiQ37ocZx2
// https://solscan.io/tx/4Cgipr3V7qgxmAFNv4ZqxowgBRxBiUtsGE4QC3gKav7FT3GZQUuB9R6z9KBms25SdvQd6pLEhhxphyaEmDnHMXaU
