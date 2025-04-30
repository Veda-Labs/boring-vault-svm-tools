use boring_vault_svm_core::generate_vanity_keypair;
use solana_keypair::Keypair;
use solana_signer::Signer;

fn main() {
    // Try to find a keypair starting with "sol" within 1M iterations
    match generate_vanity_keypair("crispy", 1_000_000_000) {
        Ok(keypair_bytes) => {
            let keypair = Keypair::from_bytes(&keypair_bytes).unwrap();
            println!("Found vanity keypair!");
            println!("Public key: {}", keypair.pubkey());
            println!("Private key bytes: {keypair_bytes:?}");
        }
        Err(e) => println!("Error: {e}"),
    }
}
