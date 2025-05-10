use std::ptr::read_unaligned;

use anchor_lang::prelude::borsh::{self, BorshDeserialize};
use eyre::{eyre, Result};
use solana_client::rpc_client::RpcClient;
use solana_pubkey::Pubkey;

pub fn get_account_data<T: BorshDeserialize>(client: &RpcClient, address: &Pubkey) -> Result<T> {
    let account = client.get_account(address)?;
    borsh::BorshDeserialize::deserialize(&mut &account.data[..])
        .map_err(|_| eyre!("Failed to deserialize data"))
}

pub fn get_account_data_unsafe<T>(client: &RpcClient, address: &Pubkey) -> Result<T> {
    let account = client.get_account(address)?;
    // Skip the 8-byte anchor discriminator and read the struct
    // note that if the account is not an anchor account, we will have issues
    Ok(unsafe { read_unaligned(account.data[8..].as_ptr() as *const T) })
}
