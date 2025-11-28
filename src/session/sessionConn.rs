use crate::error::HomeDirErr;
use crate::{
    error,
    storage::{init::PARENT_FD_NAME, types::Register},
};
use bincode::error as bin_err;
use std::{
    env,
    path::{Path, PathBuf},
};

pub struct SessionConn {
    // May be connected, may not.
    current_connected_register: Option<Register>,
    // Path to the current active vault.
    base_path: PathBuf,
}

impl SessionConn {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let home_dir = dirs_next::home_dir().ok_or(HomeDirErr::InvalidHomeDir)?;
        Ok(Self {
            current_connected_register: None,
            // No connection yet! Wrap the ROOT folder until we establish a connection.
            base_path: home_dir.join(PARENT_FD_NAME),
        })
    }

    pub fn connect_to(&mut self, register: Register) {
        self.current_connected_register = Some(register);
    }

    pub fn is_connected(&self) -> bool {
        self.current_connected_register.is_some()
    }

    pub fn get_connected_reg_name(&self) -> Option<&str> {
        match self.current_connected_register.as_ref() {
            Some(reg) => Some(reg.r_name.as_str()),
            None => None,
        }
    }

    pub fn get_reg_as_immt(&self) -> Result<&Register, ()> {
        // TODO()! Implement SessionErr
        match self.current_connected_register.as_ref() {
            Some(reg) => Ok(&reg),
            None => Err(()),
        }
    }

    pub fn get_reg_as_mut(&mut self) -> Result<&mut Register, ()> {
        // TODO()! Implement SessionErr
        match &mut self.current_connected_register {
            Some(reg) => Ok(reg),
            _ => Err(()),
        }
    }

    pub fn disconnect_from(&mut self) {
        self.current_connected_register = None;
    }
}
