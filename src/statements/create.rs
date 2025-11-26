use crate::storage;

pub struct CreateRegExec;

impl CreateRegExec {
    pub fn execute(name: &str) -> Result<(), Box<dyn std::error::Error>> {
        /// step 1: validate if the root vault already in. if not, it will propagate an
        /// VaultNotExists Error.
        storage::vault::is_vault_exisits()?;

        storage::vault::is_child_vault_f_exists()?;

        /// Validation to make sure the name does not contain special characters.
        CreateRegExec::validate_name(name);

        /// this check if the given register name is not already in the vault
        storage::vault::register_exists(name)?;

        storage::vault::create_register(name)?;

        Ok(())
    }
    pub fn validate_name(name: &str) -> Result<(), ()> {
        Ok(())
    }
}
