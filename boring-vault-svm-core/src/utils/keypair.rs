use eyre::Result;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use solana_keypair::Keypair;
use solana_signer::Signer;

pub fn generate_vanity_keypair(prefix: &str, max_iterations: u64) -> Result<Vec<u8>> {
    let prefix = prefix.to_lowercase();
    let found = Arc::new(AtomicBool::new(false));
    let result = Arc::new(std::sync::Mutex::new(None));

    // Use Rayon's parallel iterator to process chunks of iterations
    (0..max_iterations).into_par_iter().for_each(|_| {
        // If another thread found a match, stop processing
        if found.load(Ordering::Relaxed) {
            return;
        }

        let keypair = Keypair::new();
        let pubkey = keypair.pubkey().to_string();

        if pubkey.to_lowercase().starts_with(&prefix) {
            // Try to be the first to set the result
            let mut result_guard = result.lock().unwrap();
            if result_guard.is_none() {
                *result_guard = Some(keypair.to_bytes().to_vec());
                found.store(true, Ordering::Relaxed);
            }
        }
    });

    let result_guard = result.lock().unwrap();
    match result_guard.as_ref() {
        Some(bytes) => Ok(bytes.clone()),
        None => Err(eyre::eyre!(
            "Could not find keypair with prefix '{}' after {} iterations",
            prefix,
            max_iterations
        )),
    }
}
