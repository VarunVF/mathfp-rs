use crate::ast::{Expr, LiteralValue};
use crate::token::Token;


pub fn evaluate(expr: Expr) -> f64 {
    match expr {
        Expr::Literal(LiteralValue::Number(n)) => n,
        Expr::Binary { left, op, right } => {
            let l = evaluate(*left);
            let r = evaluate(*right);
            match op {
                Token::Plus => l + r,
                Token::Minus => l - r,
                Token::Star => l * r,
                Token::Slash => l / r,
                _ => unreachable!(),
            }
        },
        _ => todo!("Handle other expressions"),
    }
}