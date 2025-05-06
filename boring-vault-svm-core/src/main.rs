use boring_vault_svm_core::transaction_builder::TransactionBuilder;

use solana_sdk::pubkey::Pubkey;
use std::fs;
use std::str::FromStr;

fn show_reserve_data_example() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = "https://api.mainnet-beta.solana.com";

    let builder = TransactionBuilder::new(rpc_url.to_string());
    println!("TransactionBuilder initialized.");

    let reserve_pubkey = Pubkey::from_str("F9HdecRG8GPs9LEn4S5VfeJVEZVqrDJFR6bvmQTi22na")?;
    println!("Fetching reserve data for: {}", reserve_pubkey);

    match builder.get_reserve(&reserve_pubkey) {
        Ok(reserve_data) => {
            println!("Successfully fetched reserve data:");
            println!("{:#?}", reserve_data);

            let pretty_data = format!("{:#?}", reserve_data);
            let output_file_path = "reserve_output.txt";

            match fs::write(output_file_path, pretty_data) {
                Ok(_) => println!("Successfully wrote reserve data to {}", output_file_path),
                Err(e) => eprintln!("Error writing reserve data to file: {}", e),
            }
        }
        Err(e) => {
            eprintln!("Error fetching reserve data: {:?}", e);
        }
    }
    Ok(())
}

fn main() {
    // // Try to find a keypair st arting with "sol" within 1M iterations
    // match generate_vanity_keypair("crispy", 1_000_000_000) {
    //     Ok(keypair_bytes) => {
    //         let keypair = Keypair::from_bytes(&keypair_bytes).unwrap();
    //         println!("Found vanity keypair!");
    //         println!("Public key: {}", keypair.pubkey());
    //         println!("Private key bytes: {keypair_bytes:?}");
    //     }
    //     Err(e) => println!("Error: {e}"),
    // }

    // --- Show Reserve Data Example ---
    println!("\n--- Showing Reserve Data Example ---");
    if let Err(e) = show_reserve_data_example() {
        eprintln!("Error in reserve data example: {}", e);
    }
}
