use std::{
    fs::OpenOptions,
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
};

use super::create;
use crate::{
    encryption::{aead::decrypt, enc_utl::KdfMode, kdf::derive_slow_key, kdf::derive_fast_key},
    error,
    session::SessionConn,
    storage::{
        self,
        childvault::{self, VAULT_N},
        init::PARENT_FL_NAME,
        types::Register,
        vault_utl::{get_root_file, get_salt},
    },
};
type DynErr = Box<dyn std::error::Error>;

pub struct VaultConnection;
impl VaultConnection {
    pub fn execute(reg_name: &str, session: &mut SessionConn) -> Result<(), DynErr> {
        // Since the logic of validation remains the same for both creating and connecting to the register,
        // It's generally better to reuse function.
        create::CreateRegExec::pre_validation(reg_name, &session)?;

        // Check if Parent Exists
        //            \
        //             \
        //              \
        //            P_VAULT
        //
        //
        // If not, return an error stating that the repository needs to be initialized (INIT).
        //
        storage::vault_utl::is_parent_vault_exisits()?;

        // Check if Parent
        //           \
        //            \
        //             \
        //          P_VAULT Exists
        //
        // It's highly unlikely that this function will return an error. However, if it does,
        // it suggests someone is manipulating the binary file or has removed it entirely.
        // A possible solution is to iterate over all folders in the root directory,
        // decrypt their hashes, and add them to R_VAULT as new keys (ORDER DOES NOT MATTER).
        // This task can be implemented later.
        let parent_p = storage::vault_utl::is_parent_f_exists()?;

        // Since the parent folder and its files both exist,
        // we need to validate the file against the MAGIC and the VERSION.
        //
        storage::vault_utl::validate_f_header(&parent_p)?;
        // Using a create validation function here would be missing,
        // the first one (which belongs to Create)
        // returns AlreadyExistsErr with a key if it's successful,
        // but this one should check for non-existence.
        // That's why using a new version is much better.
        //
        //
        //
        // Maybe using an Enum Variant as output would be a good option?
        // If it exists, return Exists() (empty) and if not, return NotExists(key).
        // We should unwrap the error if we initially thought it didn't exist but found it,
        // and unwrap NotExists to get the key in that case.
        // I'll leave this to do for later improvement, lets stick with this one for a moment.

        // We need to get the key here in case the function fails
        // to reach it so that we can properly deallocate the register
        let in_key = VaultConnection::is_register_exisits(reg_name)?;

        // Seeking to register with its path or return an error,
        // the path is required to decrypt the ciphertext later.
        let child_p = VaultConnection::seek_the_request_vault(reg_name, in_key)?;

        // We validate the child file path as well to prevent
        // reading unknown or unmatched file types.
        storage::vault_utl::validate_f_header(&child_p)?;

        let bytes_data = VaultConnection::connect(&child_p)?;

        let reg = VaultConnection::load_register(bytes_data)?;

        session.connect_to(reg);

        Ok(())
    }
    pub fn is_register_exisits(reg_name: &str) -> Result<[u8; 32], DynErr> {
        let lower_reg_name = reg_name.to_lowercase();
        let parent_file_p = get_root_file()?;
        let mut root_vault = OpenOptions::new()
            .read(true)
            .open(parent_file_p.join(PARENT_FL_NAME))?;

        // The Vault is composed of 4 fields.
        // [M 4 bytes] [V 2 bytes] [S 16 Bytes] [N 2 bytes]
        //   MAGIC       VERSION       SALT      N_OF_REGS
        // We can easily jump to the exact location where
        // the number of registers is stored.
        root_vault.seek(SeekFrom::Start((22)))?;
        let mut n_of_regs_buffer = [0u8; 2];
        root_vault.read_exact(&mut n_of_regs_buffer)?;
        let in_key = derive_fast_key(&lower_reg_name, &get_salt()?);

        // Fortunately, if there are no keys, the program won't crash
        // thanks to the first two functions that ensure the existence
        // of the root and root file. If there are keys, it simply means
        // there are no registers yet, not a problem with the VAULT.
        for _ in 0..u16::from_le_bytes(n_of_regs_buffer) {
            let mut out_key = [0u8; 32];
            root_vault.read_exact(&mut out_key)?;

            if in_key == out_key {
                return Ok(in_key);
            }
        }
        return Err(Box::new(error::ConnectionErr::VaultInvalidConnection(
            (reg_name.to_string()),
        )));
    }

    pub fn seek_the_request_vault(reg_name: &str, key: [u8; 32]) -> Result<PathBuf, DynErr> {
        let lower_reg_name = reg_name.to_lowercase();
        let f_hash = format!(".{}", hex::encode(key));
        let parent_p = get_root_file()?;
        let req_p = PathBuf::from(parent_p).join(f_hash).join(VAULT_N);
        if !req_p.try_exists()? {
            return Err(Box::new(error::VaultValidationErr::UnableToSeekVault));
        }
        // deallocated the key later.
        Ok(req_p)
    }

    // Bug Fix: the key that was used as an input to ChaChaPoly was from the register name not the actual password, which lead to not accepting the password of the vault

    pub fn connect(p: &PathBuf) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut vault = OpenOptions::new().read(true).write(true).open(p)?;
        let mut salt = childvault::Vault::get_child_salt(&p)?;
        let mut nonce = childvault::Vault::get_child_nonce(&p)?;
        let mut encrypted = Vec::new();
        vault.seek(SeekFrom::Start(22 + 12))?;
        vault.read_to_end(&mut encrypted);
        let password = rpassword::prompt_password("Enter the vault's password: ")?;
        let in_key = derive_slow_key(&password, &salt);
        let _e_data = decrypt(in_key, nonce, encrypted)?;
        Ok(_e_data)
    }

    pub fn load_register(bytes_data: Vec<u8>) -> Result<Register, Box<dyn std::error::Error>> {
        let decoded: Register = {
            let config = bincode::config::standard();
            let (value, len): (Register, usize) = bincode::decode_from_slice(&bytes_data, config)?;
            value
        };

        Ok(decoded)
    }
}
