#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(u64),
    Variable(String),
    BExpr(Box<Expr>, BOp, Box<Expr>),
    UExpr(UOp, Box<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BOp {
    Mul,
    Div,
    Add,
    Sub,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Mod
}

#[derive(Debug, PartialEq, Clone)]
pub enum UOp {
    Not,
    Neg
}