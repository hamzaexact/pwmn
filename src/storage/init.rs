use crate::error::InitErr;
use crate::storage::vault::RootValut as RV;
use std::{
    env,
    fs::{self, create_dir_all as mksafe_dir},
    path::{self, Path},
};

const ROOT_FNAME: &str = "./pwmn";
const FNAME: &str = "rvault.bin"; // root vault

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let home = env::var("HOME")?;
    let path_loc = home + "/.pwmn";
    let path = Path::new(&path_loc);
    if path.try_exists()? {
        return Err(Box::new(InitErr::VaultAlreadyExists));
    }
    mksafe_dir(&path_loc);

    let s_msg = format!("initialzed empty vault repository in {}", path_loc);
    println!("{s_msg}");
    Ok(())
}
