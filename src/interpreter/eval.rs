use crate::interpreter::ast::{self, Stmt};
use crate::storage::init;
pub trait eval {
    fn eval(&self) -> Result<(), Box<dyn std::error::Error>>;
}

impl eval for Stmt {
    fn eval(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Init => init::init()?,
            other => {
                println!("TODO -> {:?}", other);
            }
        }
        Ok(())
    }
}
