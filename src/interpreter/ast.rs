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


// todo()! make this implement the COPY TRAIT

#[derive(Debug, Clone)]
pub enum Stmt {
    Empty,
    Init,
    Create { reg_name: String },
    Connect { reg_name: String },
    Select { cols: Box<Expr> },
}
