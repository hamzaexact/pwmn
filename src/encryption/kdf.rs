use crate::session;
use argon2::password_hash::rand_core::{CryptoRng, OsRng, RngCore};
use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::Aead;
const MEMORY_KiB: u32 = 0xFFFF;
const TIME_COST: u32 = 1;
const PARALLELISM: u32 = 1;
const MAGIC: &[u8; 4] = b"PWMN";

pub fn derive_key(str: &str, salt: &[u8]) -> [u8; 32] {
    let param = Params::new(MEMORY_KiB, TIME_COST, PARALLELISM, None).unwrap();

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, param);
    let mut output = [0u8; 32];
    argon2
        .hash_password_into(&str.as_bytes(), salt, &mut output)
        .unwrap();
    output
}
