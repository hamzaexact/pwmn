use crate::encryption::kdf::derive_key;
use crate::storage;
use bincode;
use rpassword;
use serde::{Deserialize, Serialize};
use std::env::SplitPaths;
use std::fs::OpenOptions;
use std::path::PathBuf;
use storage::types::Register;
use zeroize::Zeroize;

pub struct CreateRegExec;
use crate::storage::childvault::{self, ChildRootVault};

impl CreateRegExec {
    pub fn execute(name: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Step 1: Validate if the root vault exists. If not, it propagates a VaultNotExists error."
        storage::vault::is_vault_exisits()?;

        storage::vault::is_child_vault_f_exists()?;

        // Store the key to avoid re-computation in subsequent function calls.
        let key = storage::vault::register_exists(name)?;

        storage::vault::create_unique_reg_f(key)?;

        let path = childvault::ChildRootVault::new(key)?;

        storage::vault::add_to_root_vault(name)?;

        let bytes = CreateRegExec::insert_encrypted_empty_data(path, name)?;

        Ok(())
    }

    pub fn insert_encrypted_empty_data(
        p: PathBuf,
        name: &str,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut file = OpenOptions::new().read(true).write(true).open(&p)?;
        // To p, because we need to move it later to fetch the private vault salt.
        let mut password = rpassword::prompt_password(
            "A password is required to create the vault.\nPlease enter a password: ",
        )?;
        let salt = childvault::ChildRootVault::get_private_salt(p)?;
        let key = derive_key(&password, &salt);
        // Zeroize the password from memory.
        Zeroize::zeroize(&mut password);

        // Create an empty new record/register to use it later for CRUD operations
        let reg = Register::new(name);

        // Convert the register into bytes and prepare it for encryption.
        let reg_to_bytes: Vec<u8> = bincode::encode_to_vec(reg, bincode::config::standard())?;

        Ok(reg_to_bytes)
    }
}
