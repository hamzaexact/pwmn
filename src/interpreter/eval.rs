use crate::interpreter::ast::{self, DropTree, Stmt};
use crate::session::sessionConn::SessionConn;
use crate::statements::{connect, create, drop};
use crate::storage::init;
pub trait eval {
    fn eval(self, session: &mut SessionConn) -> Result<(), Box<dyn std::error::Error>>;
}

impl eval for Stmt {
    fn eval(self, session: &mut SessionConn) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Init => init::init()?,
            Self::Create { reg_name } => create::CreateRegExec::execute(&reg_name, session)?,
            Self::Connect { reg_name } => connect::VaultConnection::execute(&reg_name, session)?,
            Self::DropTree(DropTree::Reg(s)) => drop::Drop::execute(DropTree::Reg(s), &session)?,
            Self::DropTree(DropTree::Ent(s)) => drop::Drop::execute(DropTree::Ent(s), &session)?,
            other => {
                println!("TODO -> {:?}", other);
            }
        }
        Ok(())
    }
}
