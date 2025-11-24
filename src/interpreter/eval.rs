use crate::interpreter::ast::{self, Stmt};

pub trait eval {
    fn eval(&mut self) -> Result<(), ()> {
        todo!()
    }
}

impl eval for Stmt {
    fn eval(&mut self) -> Result<(), ()> {
        Ok(())
    }
}
