use crate::error::CreateErr;
use crate::error::HomeDirErr;
use crate::storage::init::{PARENT_FD_NAME, PARENT_FL_NAME};
use crate::storage::vault_utl::is_parent_vault_exisits;
use argon2::password_hash::rand_core::{CryptoRng, OsRng, RngCore};
use chacha20poly1305::{AeadCore, ChaCha20Poly1305};
use hex;
use rand::random;
use rpassword;
use std::io::{Read, Seek, SeekFrom, Write};
use std::ptr::hash;
use std::{
    env,
    fs::{self, File, OpenOptions, create_dir_all},
    path::{Path, PathBuf},
};

pub const VAULT_N: &str = "vault.bin";

type DynamicError = Box<dyn std::error::Error>;

#[derive(Debug)]
pub struct Vault {
    pub magic: [u8; 4],
    pub version: u16,
    pub salt: [u8; 16],
    pub nonce: [u8; 12],
}
impl Vault {
    pub fn new(key: [u8; 32]) -> Result<PathBuf, DynamicError> {
        let nonce_array = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let mut f = Self {
            magic: [0x50, 0x57, 0x4D, 0x4E],
            version: 1,
            salt: rand::random(),
            nonce: [0u8; 12],
        };
        f.nonce = nonce_array.into();
        let path = f.allocate_header(key)?;
        Ok(path)
    }

    pub fn allocate_header(&mut self, key: [u8; 32]) -> Result<PathBuf, DynamicError> {
        let home = dirs_next::home_dir().ok_or(HomeDirErr::InvalidHomeDir)?;
        let hash_key = format!("{}{}", ".", hex::encode(key));
        let root_file = PathBuf::from(&home)
            .join(PARENT_FD_NAME)
            .join(hash_key)
            .join(VAULT_N);
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&root_file)?;
        let mut buffer: Vec<u8> = vec![];
        buffer.extend_from_slice(&self.magic); // magic 4 bytes 
        buffer.extend_from_slice(&self.version.to_le_bytes()); // version 2 bytes
        buffer.extend_from_slice(&self.salt); // salt 16 bytes needs later to derive reg hash
        buffer.extend_from_slice(&self.nonce); // 12 to_le_bytes

        /// number of registers 2 bytes, we need this to iterate over N number of registers  to
        /// match the given input against the stored registers, since the registers would be
        /// represented as HASH not as STRING, its okay to be public not encrypted;
        file.write_all(&buffer); // 34 bytes [4][2][16][12]
        //
        Ok(root_file)
    }

    pub fn get_child_salt(p: &PathBuf) -> Result<[u8; 16], DynamicError> {
        let mut file = OpenOptions::new().read(true).open(p)?;
        file.seek(SeekFrom::Start((6)))?;
        let mut salt = [0u8; 16];
        file.read_exact(&mut salt);
        Ok(salt)
    }

    pub fn get_child_nonce(p: &PathBuf) -> Result<[u8; 12], DynamicError> {
        let mut r_vault = OpenOptions::new().read(true).open(p)?;
        // [4] [2] [16] [12]
        //  4---6---22---34
        // Basic position calculation by using prefix sum
        r_vault.seek(SeekFrom::Start((22)))?;
        let mut nonce_buf = [0u8; 12];
        r_vault.read_exact(&mut nonce_buf)?;

        Ok(nonce_buf)
    }
}
