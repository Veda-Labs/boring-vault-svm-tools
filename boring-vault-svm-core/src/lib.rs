#![allow(clippy::too_many_arguments)]
pub mod config;
pub mod instructions;
pub mod manage_instructions;
pub mod transaction_builder;
pub mod utils;

use eyre::Result;
use rayon::prelude::*;
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub enum KeypairOrPublickey {
    Keypair(Keypair),
    Publickey(Pubkey),
}

impl KeypairOrPublickey {
    pub fn pubkey(&self) -> Pubkey {
        match self {
            Self::Keypair(keypair) => keypair.pubkey(),
            Self::Publickey(pubkey) => *pubkey,
        }
    }

    pub fn into_keypair(self) -> Option<Keypair> {
        match self {
            Self::Keypair(keypair) => Some(keypair),
            Self::Publickey(_) => None,
        }
    }

    pub fn can_sign(&self) -> bool {
        matches!(self, Self::Keypair(_))
    }
}

/// Generates a vanity keypair with a desired prefix
///
/// # Arguments
///
/// * `prefix` - The desired prefix for the public key
/// * `max_iterations` - Maximum number of attempts before giving up
///
/// # Returns
///
/// Returns a Result containing the keypair bytes if successful, or an error if no matching keypair is found
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

    // Extract the result
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
