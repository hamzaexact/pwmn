use std::{
    fs::OpenOptions,
    io::{Read, Seek, SeekFrom},
};

use super::create;
use crate::{
    encryption::kdf::derive_key,
    error,
    session::SessionConn,
    storage::{
        self,
        init::FNAME,
        vault::{get_root_file, get_salt},
    },
};
type DynErr = Box<dyn std::error::Error>;

pub struct VaultConnection;
impl VaultConnection {
    pub fn execute(reg_name: &str, session: &mut SessionConn) -> Result<(), DynErr> {
        // Since the logic of validation remains the same for both creating and connecting to the register,
        // It's generally better to reuse function.
        create::CreateRegExec::pre_validation(reg_name, &session)?;

        // Check if ROOT Exists
        //            \
        //             \
        //             R_VAULT
        //
        //
        // If not, return an error stating that the repository needs to be initialized (INIT).
        storage::vault::is_vault_exisits()?;

        // Check if ROOT
        //          \
        //           \
        //            \
        //            R_VAULT Exists
        // It's highly unlikely that this function will return an error. However, if it does,
        // it suggests someone is manipulating the binary file or has removed it entirely.
        // A possible solution is to iterate over all folders in the root directory, 
        // decrypt their hashes, and add them to R_VAULT as new keys (ORDER DOES NOT MATTER). 
        // This task can be implemented later.
        storage::vault::is_child_vault_f_exists()?;

        // Using a create validation function here would be missing,
        // the first one (which belongs to Create)
        // returns AlreadyExistsErr with a key if it's successful,
        // but this one should check for non-existence.
        // That's why using a new version is much better.
        //
        //
        //
        // Maybe using an Enum Variant as output would be a good option?
        // If it exists, return Exists() (empty) and if not, return NotExists(key).
        // We should unwrap the error if we initially thought it didn't exist but found it,
        // and unwrap NotExists to get the key in that case.
        // I'll leave this to do for later improvement, lets stick with this one for a moment.
        VaultConnection::is_register_exisits(reg_name)?;

        Ok(())
    }
    pub fn is_register_exisits(reg_name: &str) -> Result<(), DynErr> {
        let lower_reg_name = reg_name.to_lowercase();
        let parent_file_p = get_root_file()?;
        let mut root_vault = OpenOptions::new()
            .read(true)
            .open(parent_file_p.join(FNAME))?;
        root_vault.seek(SeekFrom::Start((22)))?;
        let mut n_of_regs_buffer = [0u8; 2];
        root_vault.read_exact(&mut n_of_regs_buffer)?;
        let in_key = derive_key(&lower_reg_name, &get_salt()?);
        for _ in 0..u16::from_le_bytes(n_of_regs_buffer) {
            let mut out_key = [0u8; 32];
            root_vault.read_exact(&mut out_key)?;

            if in_key == out_key {
                return Ok(());
            }
        }

        return Err(Box::new(error::ConnectionErr::VaultInvalidConnection(
            (reg_name.to_string()),
        )));
    }
}
