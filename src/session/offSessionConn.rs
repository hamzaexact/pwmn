use crate::storage::{init::ROOT_FDNAME, types::Register};
use std::{
    env,
    path::{Path, PathBuf},
};

pub struct OffSessionConn {
    current_connected_register: Option<Register>, // might be connected ,might be not
    base_path: PathBuf, // path to the current active vault
}

impl OffSessionConn {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            current_connected_register: None,
            base_path: PathBuf::from(env::var("HOME")?).join(ROOT_FDNAME),
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
        match self.current_connected_register.as_ref() {
            Some(reg) => Ok(&reg),
            None => Err(()),
        }
    }

    pub fn get_reg_as_mut(&mut self) -> Result<&mut Register, ()> {
        // TODO()! SessionErr
        match &mut self.current_connected_register {
            Some(reg) => Ok(reg),
            _ => Err(()),
        }
    }

    pub fn disconnect_from(&mut self) {
        self.current_connected_register = None;
    }
}
