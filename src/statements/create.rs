use crate::encryption::kdf;
use crate::error::{CreateErr, SessionErr};
use crate::session::SessionConn;
use crate::storage::init::{FNAME, ROOT_FDNAME};
use crate::storage::{self, vault};
use crate::{encryption::kdf::derive_key, storage::rootvault::RootValut};
use bincode;
use rpassword;
use serde::{Deserialize, Serialize};
use std::env::{self, SplitPaths};
use std::fs::{OpenOptions, create_dir_all as mksafe_dir};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use storage::types::Register;
type DynError = Box<dyn std::error::Error>;
use crate::encryption::aead;
use zeroize::Zeroize;
pub struct CreateRegExec;
use crate::storage::childvault::{self, ChildRootVault};
impl CreateRegExec {
    pub fn execute(name: &str, session: &SessionConn) -> Result<(), Box<dyn std::error::Error>> {
        // Validate the input before proceeding.
        CreateRegExec::pre_validation(name, session)?;

        // "Validate if the root vault exists. If not, propagate a VaultNotExists error."
        storage::vault::is_vault_exisits()?;

        storage::vault::is_child_vault_f_exists()?;

        // Store the key to avoid re-computation in subsequent function calls.
        let key = CreateRegExec::register_exists(name)?;

        CreateRegExec::create_unique_reg_f(key)?;

        let path = childvault::ChildRootVault::new(key)?;

        storage::vault::add_to_root_vault(name)?;

        let data_as_bytes = CreateRegExec::insert_encrypted_empty_data(&path, name)?;

        let nonce = ChildRootVault::get_child_nonce(&path)?;

        let ciphertext = aead::encrypt(key, nonce, data_as_bytes)?;

        CreateRegExec::write_encrypted_data(&path, ciphertext)?;

        println!(
            "\nVault Created Successfully!\nUse CONNECT <{}> to connect to your register",
            name
        );
        Ok(())
    }

    pub fn pre_validation(name: &str, session: &SessionConn) -> Result<(), DynError> {
        // Validate the name's length first.
        if name.len() < 5 {
            return Err(Box::new(CreateErr::ShortLenErr {
                temp: "'register'".to_string(),
                target_len: 5,
            }));
        }

        // Validate the session to continue with further steps, there should be no other connection.

        if session.is_connected() {
            // returns bool
            return Err(Box::new(SessionErr::AnotherSessionIsRunningErr));
        }
        Ok(())
    }
    pub fn insert_encrypted_empty_data(
        p: &PathBuf,
        name: &str,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut file = OpenOptions::new().read(true).write(true).open(&p)?;
        // To p, because we need to move it later to fetch the private vault salt.
        let mut password = rpassword::prompt_password(
            "\nA password is required to create the vault.\nPlease enter a password: ",
        )?;

        if password.len() < 8 {
            return Err(Box::new(CreateErr::ShortLenErr {
                temp: "'password'".to_string(),
                target_len: 8,
            }));
        }
        let salt = childvault::ChildRootVault::get_child_salt(p)?;
        let key = derive_key(&password, &salt);
        // Zeroize the password from memory.
        Zeroize::zeroize(&mut password);

        // Create an empty new record/register to use it later for CRUD operations
        let reg = Register::new(name);

        // Convert the register into bytes and prepare it for encryption.
        let reg_to_bytes: Vec<u8> = bincode::encode_to_vec(reg, bincode::config::standard())?;

        Ok(reg_to_bytes)
    }

    pub fn register_exists(name: &str) -> Result<[u8; 32], Box<dyn std::error::Error>> {
        let name = name.to_lowercase();
        let root_folder = storage::vault::get_root_file()?;
        let mut vault_f = OpenOptions::new()
            .read(true)
            .write(true)
            .open(root_folder.join(FNAME))?;
        vault_f.seek(SeekFrom::Start((22)));
        let mut n_entries_buffer = [0u8; 2];
        vault_f.read_exact(&mut n_entries_buffer);
        let n_entries = u16::from_le_bytes(n_entries_buffer);
        let out_key = kdf::derive_key(name.as_str(), &vault::get_salt()?);
        if n_entries == 0 {
            return Ok(out_key);
        }
        for _ in 0..n_entries {
            let mut current_cell_key = [0u8; 32];
            vault_f.read_exact(&mut current_cell_key)?;
            if out_key == current_cell_key {
                return Err(Box::new(CreateErr::RegisterAlreadyExists));
            }
        }
        Ok(out_key)
    }
    pub fn create_unique_reg_f(key: [u8; 32]) -> Result<(), Box<dyn std::error::Error>> {
        let hash_key = format!("{}{}", ".", hex::encode(key));
        let home = env::var("HOME")?;
        let curr_reg_folder = PathBuf::from(&home).join(ROOT_FDNAME).join(hash_key);
        mksafe_dir(curr_reg_folder)?;
        Ok(())
    }
    pub fn write_encrypted_data(p: &PathBuf, ciphertext: Vec<u8>) -> Result<(), DynError> {
        let mut r_vault = OpenOptions::new().write(true).read(true).open(p)?;
        r_vault.write_all(&ciphertext)?;
        // Flushing data slowly to disk but because we're during a critical moment,
        // we need to force the flush to ensure that all data has been written.
        r_vault.flush()?;
        Ok(())
    }
}
