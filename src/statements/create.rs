use crate::encryption::enc_utl::KdfMode;
use crate::encryption::kdf;
use crate::error::{self, CreateErr, SessionErr};
use crate::session::SessionConn;
use crate::storage::enc_auth::Auth;
use crate::storage::init::ROOT_REG;
use crate::storage::vaultmod::VaultMod;
use crate::storage::{self, vaultmod};
use crate::{encryption::kdf::derive_fast_key, encryption::kdf::derive_slow_key};
use bincode;
use rpassword;
use serde::{Deserialize, Serialize};
use std::env::{self, SplitPaths};
use std::fs::{OpenOptions, create_dir_all as mksafe_dir, remove_dir_all};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use storage::types::Register;
type DynError = Box<dyn std::error::Error>;
use crate::encryption::aead;
use zeroize::Zeroize;
pub struct CreateRegExec;
use crate::storage::vault::{self, Vault};
use crate::storage::vaultmanager::VaultManager;

pub enum WriteMode {
    Vault,
    Auth,
}

impl CreateRegExec {
    pub fn execute(
        reg_name: &str,
        session: &SessionConn,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Validate the input before proceeding.
        CreateRegExec::pre_validation(reg_name, session)?;

        // "Validate if the root vault exists. If not, propagate a VaultNotExists error."
        let vault_manager = VaultManager::load()?;

        let mut child = vault_manager.create_child(reg_name)?;

        let mut vault = child.allocate()?;

        // TODO()! -> need to modify this function for the given situation.
        // We create a unique folder and add it to the parent vault,
        // which may cause other functions to fail.
        // This would result in writing a garbage key
        // that wastes space and prevents creating another
        // register with the same name.

        let (data_as_bytes, pwd_key) =
            CreateRegExec::insert_encrypted_empty_data(&mut vault, reg_name)?;

        let nonce = vault.load_nonce()?;

        let ciphertext = aead::encrypt(pwd_key, nonce, data_as_bytes)?;

        CreateRegExec::write_encrypted_data(
            vault.pathfP.as_ref().unwrap(),
            &ciphertext,
            WriteMode::Vault,
        )?;

        Auth::create_at(&vault.p)?;

        let auth = Auth::load(&vault.p)?;

        CreateRegExec::write_encrypted_data(&auth.file, &ciphertext, WriteMode::Auth);

        println!(
            "\nVault Created Successfully!\nUse CONNECT '{}' to connect to your register",
            reg_name
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
        vault: &mut VaultMod,
        name: &str,
    ) -> Result<(Vec<u8>, [u8; 32]), Box<dyn std::error::Error>> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(vault.pathfP.as_ref().unwrap())?;
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
        let salt = vault.load_salt()?;
        let key = derive_slow_key(&password, &salt);

        // Zeroize the password from memory.
        Zeroize::zeroize(&mut password);

        // Create an empty new record/register to use it later for CRUD operations
        let reg = Register::new(name);

        // Convert the register into bytes and prepare it for encryption.
        let reg_to_bytes: Vec<u8> = bincode::encode_to_vec(reg, bincode::config::standard())?;

        Ok((reg_to_bytes, key))
    }

    pub fn write_encrypted_data(
        p: &PathBuf,
        ciphertext: &Vec<u8>,
        write_mode: WriteMode,
    ) -> Result<(), DynError> {
        let pos: u64;
        match write_mode {
            WriteMode::Vault => {
                pos = 34;
            }
            WriteMode::Auth => {
                pos = 0;
            }
        }

        let mut r_vault = OpenOptions::new().write(true).read(true).open(p)?;
        // [4] [2] [16] [12]
        //  4---6---22---34
        // Position 34 marks the beginning of the empty dataset.
        r_vault.seek(SeekFrom::Start((pos)));
        r_vault.write_all(&ciphertext)?;
        // Flushing data slowly to disk but because we're during a critical moment,
        // we need to force the flush to ensure that all data has been written.
        r_vault.flush()?;
        Ok(())
    }
}
