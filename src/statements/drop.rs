// use std::{
//     fs::OpenOptions,
//     io::{Read, Seek, Write},
// };

// use anyhow::ensure;
// use serde::de::IntoDeserializer;

// use crate::interpreter::ast::DropTree;
// use crate::session::{SessionConn, sessionConn};
// use crate::{
//     encryption::kdf::derive_fast_key,
//     error::{
//         DropErr::{self},
//         SessionErr::AnotherSessionIsRunningErr,
//     },
// };

// pub struct Drop;

// impl Drop {
//     pub fn execute(
//         obj_drp: DropTree,
//         session: &sessionConn::SessionConn,
//     ) -> Result<(), Box<dyn std::error::Error>> {
//         match obj_drp {
//             DropTree::Reg(s) => {
//                 return Ok(Drop::drop_reg(&s, &session)?);
//             }
//             DropTree::Ent(s) => todo!(),
//         };
//         Ok(())
//     }
//     pub fn drop_reg(
//         reg_name: &str,
//         session: &SessionConn,
//     ) -> Result<(), Box<dyn std::error::Error>> {
//         if session.is_connected() {
//             return Err(Box::new(AnotherSessionIsRunningErr));
//         }
//         let p = validate_and_get_file(GetFile::ParentFile)?;
//         let mut pfile = OpenOptions::new().read(true).write(true).open(p)?;
//         let null_bytes = [0u8; 32];
//         let reg_key = derive_fast_key(reg_name, &get_salt()?);
//         let mut n_of_keys = [0u8; 2];
//         pfile.seek(std::io::SeekFrom::Start(22))?;
//         pfile.read_exact(&mut n_of_keys)?;
//         let mut pos = -1;
//         for i in 0..u16::from_le_bytes(n_of_keys) {
//             let mut out_key = [0u8; 32];
//             pfile.read_exact(&mut out_key);
//             if reg_key == out_key {
//                 pos = i.into();
//             }
//         }
//         if pos == -1 {
//             return Err(Box::new(DropErr::VaultNotExists {
//                 vault: reg_name.into(),
//             }));
//         }
//         pfile.seek(std::io::SeekFrom::Start((22 + 32 * i64::from(pos)) as u64))?;

//         pfile.write_all(&null_bytes);
//         println!("DONE");
//         Ok(())
//     }
// }
