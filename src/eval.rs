use crate::ast::{Expr, LiteralValue};
use crate::token::TokenType;

pub fn evaluate(expr: Expr) -> f64 {
    match expr {
        Expr::Literal(LiteralValue::Number(n)) => n,
        Expr::Binary { left, op, right } => {
            let l = evaluate(*left);
            let r = evaluate(*right);
            match op.kind {
                TokenType::Plus => l + r,
                TokenType::Minus => l - r,
                TokenType::Star => l * r,
                TokenType::Slash => l / r,
                _ => unreachable!(),
            }
        }
        Expr::Grouping(expr) => evaluate(*expr),
        _ => todo!("Handle other expressions"),
    }
}
