use Stmt::*;

use crate::error::ParserErr;

pub trait Inner {
    fn extract(&self) -> Result<Stmt, ParserErr> {
        Ok(Stmt::Empty)
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Empty,
    Number(i32),
    StringLitteral(String),
    Bool(bool),
    Identifier(String),
    Add(Box<Expr>, Box<Expr>),
    Substract(Box<Expr>, Box<Expr>),
    Devide(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Statment(Stmt),
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOperator,
        right: Box<Expr>,
    },

    And {
        left: Box<Expr>,
        right: Box<Expr>,
    },

    Or {
        left: Box<Expr>,
        right: Box<Expr>,
    },
}
#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Eq,
    NotEq,
    Ge,
    Le,
    Gt,
    Lt,
    Contains,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Empty,
    Init,
    Create { reg_name: String },
    Connect { reg_name: String },
    DropTree(DropTree),
    Select { cols: Box<Expr> },
}

#[derive(Debug, Clone)]
pub enum DropTree {
    Reg(String),
    Ent(String),
}

impl Inner for Expr {
    fn extract(&self) -> Result<Stmt, ParserErr> {
        match self {
            Expr::Statment(Stmt::Create { reg_name: s }) => Ok(Stmt::Create {
                reg_name: s.to_owned(),
            }),
            Expr::Statment(Stmt::Connect { reg_name: s }) => Ok(Stmt::Connect {
                reg_name: s.to_owned(),
            }),
            Expr::Statment(Stmt::Init) => Ok(Stmt::Init),
            Expr::Statment(Stmt::DropTree(DropTree::Reg(s))) => {
                Ok(Stmt::DropTree(DropTree::Reg(s.to_owned())))
            }
            Expr::Statment(Stmt::DropTree(DropTree::Ent(s))) => {
                Ok(Stmt::DropTree(DropTree::Ent(s.to_owned())))
            }
            _ => unreachable!(),
        }
    }
}
