use crate::error::{self, InitErr};
use crate::storage::parentvault::ParentVault as RV;
use std::{
    env,
    fs::{self, File, OpenOptions as OO, create_dir_all as mksafe_dir},
    path::{self, Path, PathBuf},
};

// Private root vault to handle all other child files.
pub const ROOT_FDNAME: &str = ".pwmn";
pub const FNAME: &str = "rvault.bin"; // root vault

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let home = dirs_next::home_dir().ok_or(error::HomeDirErr::InvalidHomeDir)?;

    let root_folder = home.join(ROOT_FDNAME);
    if root_folder.try_exists()? {
        return Err(Box::new(InitErr::VaultAlreadyExists));
    }
    mksafe_dir(&root_folder)?;
    RV::new()?;
    let s_msg = format!(
        "initialzed empty vault repository in {}/{}",
        env::var("HOME").unwrap(),
        ROOT_FDNAME
    );
    println!("{s_msg}");
    Ok(())
}
