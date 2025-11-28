use crate::error::{self, InitErr};
use crate::storage::parentvault::ParentVault;
use std::{
    env,
    fs::{self, File, OpenOptions as OO, create_dir_all as mksafe_dir},
    path::{self, Path, PathBuf},
};

// Private root vault to handle all other child files.
pub const PARENT_FD_NAME: &str = ".pwmn"; // Parent Folder Name, handled by INIT
pub const PARENT_FL_NAME: &str = "rvault.bin"; // Parent File Name

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let home = dirs_next::home_dir().ok_or(error::HomeDirErr::InvalidHomeDir)?;

    let root_folder = home.join(PARENT_FD_NAME);
    if root_folder.try_exists()? {
        return Err(Box::new(InitErr::VaultAlreadyExists));
    }
    mksafe_dir(&root_folder)?;
    ParentVault::new()?;
    let s_msg = format!(
        "initialzed empty vault repository in {}/{}",
        env::var("HOME").unwrap(),
        PARENT_FD_NAME
    );
    println!("{s_msg}");
    Ok(())
}
