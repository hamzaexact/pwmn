use crate::error::{self, InitErr};

use std::{
    env,
    fs::{self, File, create_dir_all as mksafe_dir},
    path::{self, Path, PathBuf},
};

// Private root vault to handle all other child files.
pub const ROOT_REG: &str = ".pwmn"; // Parent Folder Name, handled by INIT

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let home = dirs_next::home_dir().ok_or(error::HomeDirErr::InvalidHomeDir)?;

    let root_folder = home.join(ROOT_REG);
    if root_folder.try_exists()? {
        return Err(Box::new(InitErr::RootVaultAlreadyExists));
    }
    mksafe_dir(&root_folder)?;
    let s_msg = format!(
        "Initialized an empty  repository in {}/{}",
        env::var("HOME").unwrap(),
        ROOT_REG
    );
    println!("{s_msg}");
    Ok(())
}
