use rand::rngs::adapter::ReseedingRng;

// use crate::encryption::kdf;
use super::super::encryption::kdf;
use super::init::{FNAME, ROOT_FDNAME};
use crate::encryption::kdf::derive_key;
use crate::error::CreateErr;
use std::fmt::format;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::{
    env,
    fs::{File, OpenOptions, create_dir_all as mksafe_dir},
    path::PathBuf,
};

pub fn is_vault_exisits() -> Result<(), Box<dyn std::error::Error>> {
    let home = env::var("HOME")?;
    let root_folder = PathBuf::from(&home).join(ROOT_FDNAME);
    if !(root_folder.try_exists()?) {
        return Err(Box::new(CreateErr::VaultNotExists));
    }
    Ok(())
}

pub fn is_child_vault_f_exists() -> Result<(), Box<dyn std::error::Error>> {
    // if the root_folder exists but the child vault file is missed somehow
    //
    let root_folder = get_root_file()?;
    if !(root_folder.join(FNAME).try_exists()?) {
        return Err(Box::new(CreateErr::DestroyedVaultErr));
    }

    Ok(())
}

pub fn get_root_file() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home = env::var("HOME")?;
    Ok(PathBuf::from(&home).join(ROOT_FDNAME))
}

pub fn get_salt() -> Result<[u8; 16], Box<dyn std::error::Error>> {
    let root_folder = get_root_file()?;
    let mut root_vault_f = OpenOptions::new()
        .write(true)
        .read(true)
        .open(root_folder.join(FNAME))?;
    root_vault_f.seek(SeekFrom::Start((6)))?;
    let mut salt = [0u8; 16];
    root_vault_f.read_exact(&mut salt)?;
    Ok(salt)
}

pub fn add_to_root_vault(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.to_lowercase();
    // root file vault to derive the key
    let salt = get_salt()?;
    let root_folder = get_root_file()?;
    let mut root_vault_f = OpenOptions::new()
        .read(true)
        .write(true)
        .open(root_folder.join(FNAME))?;
    root_vault_f.seek(SeekFrom::Start(22))?;
    let mut n_entries_buffer = [0u8; 2];
    root_vault_f.read_exact(&mut n_entries_buffer)?;
    let n_of_entries = u16::from_le_bytes(n_entries_buffer);
    let offset = n_of_entries * 32;
    root_vault_f.seek(SeekFrom::Start(((24 + offset) as u64)))?;
    let new_register_key = derive_key(name.as_str(), &salt);
    root_vault_f.write_all(&new_register_key)?;
    root_vault_f.seek(SeekFrom::Start((22)))?;
    root_vault_f.write_all(&((n_of_entries + 1) as u16).to_le_bytes())?;
    Ok(())
}


