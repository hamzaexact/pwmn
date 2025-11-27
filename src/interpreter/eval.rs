use crate::interpreter::ast::{self, Stmt};
use crate::session::offSessionConn::SessionConn;
use crate::statements::{connect, create};
use crate::storage::init;
pub trait eval {
    fn eval(&self, session: &mut SessionConn) -> Result<(), Box<dyn std::error::Error>>;
}

impl eval for Stmt {
    fn eval(&self, session: &mut SessionConn) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Init => init::init()?,
            Self::Create { reg_name } => create::CreateRegExec::execute(reg_name, session)?,
            Self::Connect { reg_name } => connect::VaultConnection::execute(reg_name, session)?,
            other => {
                println!("TODO -> {:?}", other);
            }
        }
        Ok(())
    }
}
