use argon2::password_hash::rand_core::{CryptoRng, OsRng, RngCore};
use chacha20poly1305::aead::Aead;
use chacha20poly1305::{AeadCore, ChaCha20Poly1305 as CCP, KeyInit, aead};

pub fn encrypt(key: [u8; 32], data: &str) -> (Vec<u8>, Vec<u8>) {
    let cipher = CCP::new(&key.into());
    let nonce = CCP::generate_nonce(&mut OsRng);
    let encryped = cipher.encrypt(&nonce, data.as_bytes()).unwrap();
    (nonce.to_vec(), encryped)
}
