use eyre::Result;
use solana_client::rpc_client::RpcClient;
use solana_instruction::Instruction;

use crate::{manage_instructions::MintJitoSol, KeypairOrPublickey};

use super::create_manage_instruction;

pub fn create_mint_jito_sol_instructions(
    client: &RpcClient,
    signer: &KeypairOrPublickey,
    authority: Option<&KeypairOrPublickey>,
    vault_id: u64,
    sub_account: u8,
    amount: u64,
) -> Result<Vec<Instruction>> {
    let mut instructions = vec![];

    let eix = MintJitoSol::new(vault_id, sub_account, amount);
    instructions.extend(create_manage_instruction(client, signer, authority, eix)?);

    Ok(instructions)
}
