use crate::encryption::enc_utl::KdfMode;
use crate::session;
use argon2::password_hash::rand_core::{CryptoRng, OsRng, RngCore};
use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::Aead;
const TIME_COST: u32 = 1;
const PARALLELISM: u32 = 1;
const MAGIC: &[u8; 4] = b"PWMN";

pub fn derive_key(str: &str, salt: &[u8], kdf_mode: KdfMode) -> [u8; 32] {
    let mut m_cost: u32;
    match kdf_mode {
        KdfMode::EncrM => {
            // Faster for creating registers
            m_cost = 0x4000; // 16 * 1024; 
        }
        KdfMode::DecryM => {
            // Slower for connecting to registers (prevent fast attacks if any!).
            m_cost = 0x04_0000; // 265 * 1024;
        }
    }
    let param = Params::new(m_cost, TIME_COST, PARALLELISM, None).unwrap();

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, param);
    let mut output = [0u8; 32];
    argon2
        .hash_password_into(&str.as_bytes(), salt, &mut output)
        .unwrap();
    output
}
