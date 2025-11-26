use crate::storage;

pub struct CreateRegExec;
use crate::storage::childvault;

impl CreateRegExec {
    pub fn execute(name: &str) -> Result<(), Box<dyn std::error::Error>> {
        /// step 1: validate if the root vault already in. if not, it will propagate an
        /// VaultNotExists Error.
        storage::vault::is_vault_exisits()?;

        storage::vault::is_child_vault_f_exists()?;

        let key = storage::vault::register_exists(name)?;

        storage::vault::create_unique_reg_f(key);

        childvault::ChildRootVault::new(key)?;

        storage::vault::add_to_root_vault(name)?;

        Ok(())
    }
}
