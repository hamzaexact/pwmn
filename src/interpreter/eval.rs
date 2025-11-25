use crate::interpreter::ast::{self, Stmt};
use crate::statements::create;
use crate::storage::init;
pub trait eval {
    fn eval(&self) -> Result<(), Box<dyn std::error::Error>>;
}

impl eval for Stmt {
    fn eval(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Init => init::init()?,
            Self::Create { reg_name } => create::CreateRegExec::execute(reg_name)?,
            other => {
                println!("TODO -> {:?}", other);
            }
        }
        Ok(())
    }
}
