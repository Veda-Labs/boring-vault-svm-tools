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
    Ok(unsafe { read_unaligned(account.data.as_ptr() as *const T) })
}
