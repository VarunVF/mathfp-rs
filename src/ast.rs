use crate::token::Token;

#[derive(Clone, Debug, PartialEq)]
pub struct MatchArm {
    pub pattern: Box<Expr>,
    pub body: Box<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Program {
        statements: Vec<Expr>,
    },
    FunctionBody {
        statements: Vec<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Unary {
        op: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Variable(String),
    Assign {
        name: String,
        expr: Box<Expr>,
    },
    Binding {
        name: String,
        expr: Box<Expr>,
    },
    Literal(LiteralValue),
    FunctionDef {
        param: String,
        body: Box<Expr>,
    },
    FunctionCall {
        func: Box<Expr>,
        arg: Box<Expr>,
    },
    If {
        cond_expr: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
    },
    Match {
        arms: Vec<MatchArm>,
    },
    Empty,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}
