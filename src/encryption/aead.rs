use crate::error::{DecryptionErr, EncryptionErr};
use argon2::password_hash::rand_core::{CryptoRng, OsRng, RngCore};
use chacha20poly1305::aead::Aead;
use chacha20poly1305::{AeadCore, ChaCha20Poly1305 as CCP, KeyInit, Nonce, aead};

pub fn encrypt(key: [u8; 32], nonce: [u8; 12], data: Vec<u8>) -> Result<Vec<u8>, EncryptionErr> {
    let cipher = CCP::new(&key.into());
    let nonce_ref = Nonce::from_slice(&nonce);
    let encryped = cipher
        .encrypt(nonce_ref, data.as_slice())
        .map_err(|e| EncryptionErr::EncryptionErr)?;
    Ok(encryped)
}

pub fn decrypt(
    key: [u8; 32],
    nonce: [u8; 12],
    encrypted: Vec<u8>,
) -> Result<Vec<u8>, DecryptionErr> {
    let mut cipther = CCP::new(&key.into());
    let nonce_ref = Nonce::from_slice(&nonce);
    let decrypted = cipther
        .decrypt(nonce_ref, encrypted.as_slice())
        .map_err(|e| DecryptionErr::DecryptionErr)?;
    Ok(decrypted)
}
