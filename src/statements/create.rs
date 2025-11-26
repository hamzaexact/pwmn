use rpassword;
use std::env::SplitPaths;
use std::fs::OpenOptions;
use std::path::PathBuf;
use zeroize::Zeroize;

use crate::encryption::kdf::derive_key;
use crate::storage;

pub struct CreateRegExec;
use crate::storage::childvault::{self, ChildRootVault};

impl CreateRegExec {
    pub fn execute(name: &str) -> Result<(), Box<dyn std::error::Error>> {
        /// step 1: validate if the root vault already in. if not, it will propagate an
        /// VaultNotExists Error.
        storage::vault::is_vault_exisits()?;

        storage::vault::is_child_vault_f_exists()?;

        let key = storage::vault::register_exists(name)?;

        storage::vault::create_unique_reg_f(key)?;

        let path = childvault::ChildRootVault::new(key)?;

        storage::vault::add_to_root_vault(name)?;

        Ok(())
    }

    pub fn insert_encrypted_empty_data(p: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = OpenOptions::new().read(true).write(true).open(&p)?; // pointer
        // to p, because we need to move it later to fetch the PRIVATE VAULT salt.
        let mut password =
            rpassword::prompt_password("Creating Vault required a password to enter with later: ")?;
        let salt = childvault::ChildRootVault::get_private_salt(p)?;
        let key = derive_key(&password, &salt);
        Zeroize::zeroize(&mut password); // zeroize the password in memory

        Ok(())
    }
}
