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
    pub fn new() -> Self {
        let mut f = Self {
            magic: [0x50, 0x57, 0x4D, 0x4E],
            version: 1,
            salt: rand::random(),
            n_regs: 0,
        };

        f
    }

    fn allocate_header(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let k = is_vault_exisits()?;
        let home = env::var("HOME")?;
        let root_file = PathBuf::from(&home).join(ROOT_FDNAME).join(FNAME);
        let mut file = OpenOptions::new().read(true).write(true).open(root_file)?;
        let mut buffer: Vec<u8> = vec![];
        buffer.extend_from_slice(&self.magic);
        buffer.extend_from_slice(&self.version.to_le_bytes());
        buffer.extend_from_slice(&self.salt);
        buffer.extend_from_slice(&self.n_regs.to_le_bytes());
        file.write_all(&buffer);
        Ok(())
    }
}
