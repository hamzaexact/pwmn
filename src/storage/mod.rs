pub mod init;
pub mod types;
pub mod vault;
use crate::error::CreateErr;
use init::ROOT_FDNAME;
use std::{env, path::PathBuf};

pub fn is_vault_exisits() -> Result<(), Box<dyn std::error::Error>> {
    let home = env::var("HOME")?;
    let root_folder = PathBuf::from(&home).join(ROOT_FDNAME);
    if !(root_folder.try_exists()?) {
        return Err(Box::new(CreateErr::VaultNotExists));
    }
    Ok(())
}
