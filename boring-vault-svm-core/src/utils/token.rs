use eyre::Result;
use solana_client::rpc_client::RpcClient;
use solana_program_pack::Pack;
use solana_pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address_with_program_id;
use spl_token::state::Account as TokenAccount;

pub fn get_token_account_balance(
    client: &RpcClient,
    owner: &Pubkey,
    mint: &Pubkey,
    is_token_2022: bool,
) -> Result<Option<u64>> {
    let token_program_id = if is_token_2022 {
        spl_token_2022::ID
    } else {
        spl_token::ID
    };

    let ata_address = get_associated_token_address_with_program_id(owner, mint, &token_program_id);

    let data = client.get_account(&ata_address)?.data;

    Ok(TokenAccount::unpack(&data)
        .ok()
        .map(|token_account| token_account.amount))
}
