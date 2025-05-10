use std::str::FromStr;

use boring_vault_svm_core::builder::Builder;
use eyre::Result;
use solana_pubkey::Pubkey;

fn main() -> Result<()> {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let tb = Builder::new(rpc_url.to_string(), Some("data".to_string()));

    // tb.get_sub_account_token_totals()?;

    // let data = get_asset_data(
    //     &tb.client,
    //     tb.vault_config.vault_id,
    //     Pubkey::from_str("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn")?,
    // )?;

    // let (addy, data) = get_vault_state(&tb.client, tb.vault_config.vault_id)?;

    // let reserve = tb.get_reserve(&Pubkey::from_str("6jddhtQG5ZqCbnvt6E4ckwjCECnJVLu8QabxGztCd2mD")?)?;
    let obligation = tb.get_obligation(&Pubkey::from_str(
        "6jddhtQG5ZqCbnvt6E4ckwjCECnJVLu8QabxGztCd2mD",
    )?)?;

    println!("{obligation:?}");

    Ok(())
}
