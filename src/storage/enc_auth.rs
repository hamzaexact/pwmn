use crate::{
    encryption::{aead::decrypt, kdf::derive_slow_key},
    error::{AuthErr, FileReqErr},
};
use std::{fs::OpenOptions, io::Read, path::PathBuf};

pub const AUTH: &str = "auth.pwmn";
pub struct Auth {
    pub file: PathBuf,
}

impl Auth {

    pub fn create_at(p: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = PathBuf::from(p).join(AUTH);
        OpenOptions::new().read(true).write(true).create(true).open(&file)?;
        Ok(())
    }


    pub fn load(p: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = PathBuf::from(p).join(AUTH);
        if !file
            .try_exists()
            .map_err(|e| FileReqErr::UnexpectedIOError)?
        {
            return Err(Box::new(AuthErr::AuthFileNotFound));
        }
        Ok(Self { file })
    }

    pub fn connect(
        &self,
        prompt: &str,
        salt: [u8; 16],
        nonce: [u8; 12],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = OpenOptions::new().write(true).read(true).open(&self.file)?;
        let mut buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer)?;
        let password = rpassword::prompt_password(prompt)?;
        let key = derive_slow_key(&password, &salt);
        decrypt(key, nonce, buffer)?;
        Ok(())
    }
}
