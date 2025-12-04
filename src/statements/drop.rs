use crate::interpreter::ast::DropTree;
use crate::session::{SessionConn, session_conn};
use crate::storage::enc_auth::Auth;
use crate::{
    encryption::kdf::derive_fast_key,
    error::{
        DropErr::{self},
        SessionErr::AnotherSessionIsRunningErr,
    },
};
use std::{
    fs::{OpenOptions, remove_dir_all},
    io::{Read, Seek, Write},
};

use crate::storage::vaultmanager;

pub struct Drop;

impl Drop {
    pub fn execute(
        obj_drp: DropTree,
        session: &session_conn::SessionConn,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match obj_drp {
            DropTree::Reg(s) => {
                return Ok(Drop::drop_reg(&s, &session)?);
            }
            DropTree::Ent(s) => todo!(),
        };
        Ok(())
    }
    pub fn drop_reg(
        reg_name: &str,
        session: &SessionConn,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if session.is_connected() {
            return Err(Box::new(AnotherSessionIsRunningErr));
        }
        let mut vault_manager = vaultmanager::VaultManager::load()?;
        let (_, child_p) = vault_manager.validate_register(reg_name, false)?;
        let mut vault = vault_manager.external_vault_load(&child_p)?;
        let auth = Auth::load(&vault.p)?;
        auth.connect(
            "Entre the password of the vault: ",
            vault.salt.as_ref().unwrap(),
            vault.nonce.as_ref().unwrap(),
        )?;
        remove_dir_all(vault.p);
        println!(
            "Register with name '{}' hash been successfully removed",
            reg_name
        );
        Ok(())
    }
}
