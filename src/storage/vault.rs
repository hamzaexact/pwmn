use crate::error::CreateErr;
use crate::storage::init::{FNAME, ROOT_FDNAME};
use crate::storage::is_vault_exisits;
use rand::random;
use std::io::Write;
use std::{
    env,
    fs::{self, File, OpenOptions, create_dir_all},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct RootValut {
    pub magic: [u8; 4],
    pub version: u16,
    pub salt: [u8; 16],
    pub n_regs: u16,
}

impl RootValut {
    pub fn new() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = Self {
            magic: [0x50, 0x57, 0x4D, 0x4E],
            version: 1,
            salt: rand::random(),
            n_regs: 0,
        };
        f.allocate_header()?;
        Ok(())
    }

    fn allocate_header(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let k = is_vault_exisits()?;
        let home = env::var("HOME")?;
        let root_file = PathBuf::from(&home).join(ROOT_FDNAME).join(FNAME);
        let mut file = OpenOptions::new().read(true).write(true).create(true).open(root_file)?;
        let mut buffer: Vec<u8> = vec![];
        buffer.extend_from_slice(&self.magic); // magic 4bytes 
        buffer.extend_from_slice(&self.version.to_le_bytes()); // version 2bytes
        buffer.extend_from_slice(&self.salt); // salt 16 bytes needs later to derive reg hash
        /// number of registers 2 bytes, we need this to iterate over N number of registers  to
        /// match the given input against the stored registers, since the registers would be
        /// represented as HASH not as STRING, its okay to be public not encrypted.
        buffer.extend_from_slice(&self.n_regs.to_le_bytes()); 

        file.write_all(&buffer); // 24 bytes [4][2][16][2]
        Ok(())
    }
}
