#![allow(clippy::too_many_arguments)]
pub mod builder;
pub mod config;
pub mod instructions;
pub mod manage_instructions;
pub mod transaction;
pub mod utils;
pub mod view;

use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_signer::Signer;

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
