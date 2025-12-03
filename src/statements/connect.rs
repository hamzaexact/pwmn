use std::{
    fs::{OpenOptions, write},
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
};

use super::create;
use crate::{
    encryption::{
        aead::decrypt,
        enc_utl::KdfMode,
        kdf::{derive_fast_key, derive_slow_key},
    },
    error,
    session::SessionConn,
    storage::{self, types::Register, vault::VAULT_N, vaultmod::VaultMod},
};
type DynErr = Box<dyn std::error::Error>;

pub struct VaultConnection;
impl VaultConnection {
    pub fn execute(reg_name: &str, session: &mut SessionConn) -> Result<(), DynErr> {
        // Since the logic of validation is the same for both registering and reconnecting
        // to a database or system, it's generally more efficient to reuse existing code
        // rather than re-implementing it.
        //
        //
        create::CreateRegExec::pre_validation(reg_name, &session)?;

        // Check if ROOT Exists
        //            \
        //             \
        //              \
        //          VAULT FOLDER
        //
        //
        // If not, return an error indicating that the repository must be initialized.
        //
        let manager = storage::vaultmanager::VaultManager::load()?;

        // We need to get the key here in case the function fails
        // to reach it so that we can properly deallocate the register
        let (_, child_p) = manager.validate_register(reg_name, false)?;

        // Seeking to register with its path or return an error,
        // the path is required to decrypt the ciphertext later.

        // We validate the child file path as well to prevent
        // reading unknown or unmatched file types.
        //
        let mut vault = manager.external_vault_load(&child_p)?;

        vault.validate_f_header();

        let bytes_data = VaultConnection::connect(&mut vault)?;

        let reg = VaultConnection::load_register(bytes_data)?;

        session.connect_to(reg);

        println!("CONNECTED");

        Ok(())
    }

    pub fn connect(vault_mod: &mut VaultMod) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut vault = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&vault_mod.pathfP.as_ref().unwrap())?;
        let mut salt = vault_mod.load_salt()?;
        let mut nonce = vault_mod.load_nonce()?;
        let mut encrypted = Vec::new();
        vault.seek(SeekFrom::Start(22 + 12))?;
        vault.read_to_end(&mut encrypted);
        let password = rpassword::prompt_password("Enter the vault's password: ")?;
        let in_key = derive_slow_key(&password, &salt);
        let _e_data = decrypt(in_key, nonce, encrypted)?;
        Ok(_e_data)
    }

    pub fn load_register(bytes_data: Vec<u8>) -> Result<Register, Box<dyn std::error::Error>> {
        let decoded: Register = {
            let config = bincode::config::standard();
            let (value, len): (Register, usize) = bincode::decode_from_slice(&bytes_data, config)?;
            value
        };

        Ok(decoded)
    }
}
