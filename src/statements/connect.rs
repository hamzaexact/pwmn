use std::{
    fs::OpenOptions,
    io::{Read, Seek, SeekFrom},
};

use super::create;
use crate::{
    encryption::kdf::derive_key,
    error,
    session::SessionConn,
    storage::{init::FNAME, vault::{get_root_file, get_salt}},
};
type DynErr = Box<dyn std::error::Error>;

pub struct VaultConnection;
impl VaultConnection {
    pub fn execute(reg_name: &str, session: &mut SessionConn) -> Result<(), DynErr> {
        // Since the logic of validation remains the same for both creating and connecting to the register,
        // It's generally better to reuse function.
        create::CreateRegExec::pre_validation(reg_name, &session);

        VaultConnection::is_register_exisits(reg_name)?;

        Ok(())
    }
    pub fn is_register_exisits(reg_name: &str) -> Result<(), DynErr> {
        let lower_reg_name = reg_name.to_lowercase();
        let parent_file_p = get_root_file()?;
        let mut root_vault = OpenOptions::new().read(true).open(parent_file_p.join(FNAME))?;
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
