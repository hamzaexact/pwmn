use super::vault::VAULT_N;
use super::{init::ROOT_REG, vault::Vault};
use crate::encryption::kdf::derive_fast_key;
use crate::error::{self, ConnectionErr, CreateErr, DropErr, FileReqErr};
use crate::storage::vaultmod::VaultMod;
use hex;
use std::fs::create_dir_all as mksafe_dir;
use std::path::PathBuf;

pub const SALT: [u8; 16] = [
    188, 209, 128, 213, 229, 38, 112, 152, 37, 246, 56, 123, 185, 210, 43, 26,
];

pub struct VaultManager {
    p: PathBuf,
}

impl VaultManager {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let home = dirs_next::home_dir().ok_or(error::HomeDirErr::InvalidHomeDir)?;
        let root_folder = home.join(ROOT_REG);
        if !(root_folder.try_exists()?) {
            return Err(Box::new(CreateErr::VaultNotExists));
        }

        Ok(Self { p: root_folder })
    }
    pub fn get_root_path(&self) -> &PathBuf {
        &self.p
    }

    pub fn validate_register(
        &self,
        reg_name: &str,
        to_create: bool,
    ) -> Result<(String, PathBuf), Box<dyn std::error::Error>> {
        let child = format!(".{}", hex::encode(derive_fast_key(reg_name, &SALT)));
        let path = PathBuf::from(&self.p).join(&child);
        if path
            .try_exists()
            .map_err(|E| FileReqErr::UnexpectedIOError)?
            && to_create
        {
            return Err(Box::new(CreateErr::RegisterAlreadyExists));
        } else if !path
            .try_exists()
            .map_err(|E| FileReqErr::UnexpectedIOError)?
            && !to_create
        {
            return Err(Box::new(DropErr::VaultNotExists {
                vault: reg_name.to_string(),
            }));
        }
        Ok((child, path))
    }

    pub fn create_child(&self, reg_name: &str) -> Result<VaultMod, Box<dyn std::error::Error>> {
        let (f_hex, _) = self.validate_register(reg_name, true)?;
        let target_folder = PathBuf::from(&self.p).join(&f_hex);
        mksafe_dir(&target_folder)
            .map_err(|e| "Something Went Wrong while creating the register")?;
        Ok(VaultMod {
            p: target_folder,
            pathfP: None,
            salt: None,
            nonce: None,
        })
    }

    pub fn external_vault_load(
        &self,
        child: &PathBuf,
    ) -> Result<VaultMod, Box<dyn std::error::Error>> {
        let mut vault = VaultMod {
            p: child.clone(),
            pathfP: Some(child.clone().join(VAULT_N)),
            salt: None,
            nonce: None,
        };
        vault.load_salt()?;
        vault.load_nonce()?;
        Ok(vault)
    }
}
