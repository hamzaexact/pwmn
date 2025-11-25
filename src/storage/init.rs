use crate::error::InitErr;
use crate::storage::rootvault::RootValut as RV;
use std::{
    env,
    fs::{self, File, OpenOptions as OO, create_dir_all as mksafe_dir},
    path::{self, Path, PathBuf},
};

pub const ROOT_FDNAME: &str = ".pwmn";
pub const FNAME: &str = "rvault.bin"; // root vault

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let home = env::var("HOME")?;
    // let path_loc = home + ROOT_FNAME;
    let root_folder = PathBuf::from(&home).join(ROOT_FDNAME);
    if root_folder.try_exists()? {
        return Err(Box::new(InitErr::VaultAlreadyExists));
    }
    mksafe_dir(&root_folder)?;
    RV::new()?;
    let s_msg = format!(
        "initialzed empty vault repository in {}/{}",
        home, ROOT_FDNAME
    );
    println!("{s_msg}");
    Ok(())
}
