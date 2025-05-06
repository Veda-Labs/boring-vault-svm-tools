use std::ptr::read_unaligned;

use anchor_lang::prelude::borsh::{self, BorshDeserialize};
use solana_client::rpc_client::RpcClient;
use solana_pubkey::Pubkey;
use eyre::{eyre, Result};

pub fn get_account_data<T: BorshDeserialize>(client: &RpcClient, address: &Pubkey, safe: bool) -> Result<T> {
    let account = client.get_account(address)?;

    if safe {
        borsh::BorshDeserialize::deserialize(&mut &account.data[..])
        .map_err(|_| eyre!("Failed to deserialize data"))
    } else {
        Ok( unsafe { read_unaligned(account.data.as_ptr() as *const T)})
    }
}