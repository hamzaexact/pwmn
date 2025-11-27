use crate::interpreter::ast::{self, Stmt};
use crate::session::offSessionConn::SessionConn;
use crate::statements::{create};
use crate::storage::init;
pub trait eval {
    fn eval(&self, session: &mut SessionConn) -> Result<(), Box<dyn std::error::Error>>;
}

impl eval for Stmt {
    fn eval(&self, session: &mut SessionConn) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Init => init::init()?,
            Self::Create { reg_name } => create::CreateRegExec::execute(reg_name, session)?,
            Self::Create { reg_name } => todo!(),
            other => {
                println!("TODO -> {:?}", other);
            }
        }
        Ok(())
    }
}
