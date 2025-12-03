use crate::encryption::enc_utl::KdfMode;
use crate::error::VaultValidationErr;
use anyhow::ensure;
use rand::rngs::adapter::ReseedingRng;
// use crate::encryption::kdf;
use super::super::encryption::kdf;
use super::init::ROOT_REG;
use super::vault::Vault;
use crate::encryption::kdf::{derive_fast_key, derive_slow_key};
use crate::error::{self, CreateErr};
use crate::storage::vault::VAULT_N;
use std::fmt::format;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::{
    env,
    fs::{File, OpenOptions, create_dir_all as mksafe_dir},
    path::PathBuf,
};

type DynamicErr = Box<dyn std::error::Error>;

#[derive(Debug)]
pub struct VaultMod {
    pub p: PathBuf,
    pub pathfP: Option<PathBuf>,
    pub salt: Option<[u8; 16]>,
    pub nonce: Option<[u8; 12]>,
}

impl VaultMod {
    pub fn allocate(&mut self) -> Result<&mut Self, Box<dyn std::error::Error>> {
        let allocated_path = Vault::allocate(&self.p)?;
        self.pathfP = Some(allocated_path);

        Ok(self)
    }

    pub fn load_salt(&mut self) -> Result<[u8; 16], Box<dyn std::error::Error>> {
        if self.salt.is_some() {
            return Ok(self.salt.unwrap());
        }
        let path = self.pathfP.as_ref().unwrap();
        let mut file = OpenOptions::new().read(true).open(path)?;
        file.seek(SeekFrom::Start(6))?;
        let mut salt = [0u8; 16];
        file.read_exact(&mut salt)?;
        self.salt = Some(salt);
        Ok(salt)
    }

    pub fn load_nonce(&mut self) -> Result<[u8; 12], Box<dyn std::error::Error>> {
        if self.nonce.is_some() {
            return Ok(self.nonce.unwrap());
        }
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(self.pathfP.as_ref().unwrap())?;
        file.seek(SeekFrom::Start(22))?;
        let mut nonce = [0u8; 12];
        file.read_exact(&mut nonce)?;
        self.nonce = Some(nonce);
        Ok(nonce)
    }

    pub fn validate_f_header(&self) -> Result<(), DynamicErr> {
        let mut t_file = OpenOptions::new()
            .read(true)
            .open(self.pathfP.as_ref().unwrap())?;
        t_file.seek(SeekFrom::Start(0));
        let mut t_file_magic = [0u8; 4];
        let mut t_file_version = [0u8; 2];
        t_file.read_exact(&mut t_file_magic)?;
        t_file.read_exact(&mut t_file_version)?;
        if t_file_magic != *b"PWMN" {
            return Err(Box::new(VaultValidationErr::MismatchedFileHeader));
        }

        if u16::from_le_bytes(t_file_version) != 1 {
            return (Err(Box::new(error::VaultValidationErr::MismatchedFileHeader)));
        }
        Ok(())
    }
}
