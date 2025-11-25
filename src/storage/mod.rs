pub mod init;
pub mod rootvault;
pub mod types;
pub mod vault;
// use crate::encryption::kdf;
// use crate::error::CreateErr;
// use init::FNAME;
// use init::ROOT_FDNAME;
// use std::io::Read;
// use std::io::Seek;
// use std::io::SeekFrom;
// use std::{
//     env,
//     fs::{File, OpenOptions},
//     path::PathBuf,
// };

// pub fn is_vault_exisits() -> Result<(), Box<dyn std::error::Error>> {
//     let home = env::var("HOME")?;
//     let root_folder = PathBuf::from(&home).join(ROOT_FDNAME);
//     if !(root_folder.try_exists()?) {
//         return Err(Box::new(CreateErr::VaultNotExists));
//     }
//     Ok(())
// }

// pub fn register_exists(name: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let root_folder = get_root_file()?;
//     let mut vault_f = OpenOptions::new()
//         .read(true)
//         .write(true)
//         .open(root_folder.join(FNAME))?;
//     vault_f.seek(SeekFrom::Start((22)));
//     let mut n_entries_buffer = [0u8; 2];
//     vault_f.read_exact(&mut n_entries_buffer);
//     let n_entries = u16::from_le_bytes(n_entries_buffer);
//     if n_entries == 0 {
//         return Ok(());
//     }
//     for _ in 0..n_entries {
//         let mut current_cell_key = [0u8; 32];
//         vault_f.read_exact(&mut current_cell_key)?;
//         let out_key = kdf::derive_key(name, &get_salt()?);
//         if out_key == current_cell_key {
//             return Err(Box::new(CreateErr::RegisterAlreadyExists));
//         }
//     }
//     Ok(())
// }

// pub fn get_root_file() -> Result<PathBuf, Box<dyn std::error::Error>> {
//     let home = env::var("HOME")?;
//     Ok(PathBuf::from(&home).join(ROOT_FDNAME))
// }

// pub fn get_salt() -> Result<[u8; 16], Box<dyn std::error::Error>> {
//     let root_folder = get_root_file()?;
//     let mut root_vault_f = OpenOptions::new()
//         .write(true)
//         .read(true)
//         .open(root_folder.join(FNAME))?;
//     root_vault_f.seek(SeekFrom::Start((6)))?;
//     let mut salt = [0u8; 16];
//     root_vault_f.read_exact(&mut salt)?;
//     Ok(salt)
// }
