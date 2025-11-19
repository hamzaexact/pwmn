pub enum Expr {
    Number(i32),
    String(String),
    Bool(bool),
    Identifier(String),

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
    Create { reg_name: String },
    Connect { reg_name: String },
}
