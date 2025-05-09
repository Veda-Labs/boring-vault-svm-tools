pub fn get_anchor_discriminator(name: &str) -> [u8; 8] {
    let preimage = format!("global:{name}");
    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(&solana_program::hash::hash(preimage.as_bytes()).to_bytes()[..8]);
    sighash
}
