use crate::ast::{Expr, LiteralValue};
use crate::runtime::{Environment, RuntimeValue};
use crate::token::TokenType;

pub fn evaluate(expr: Expr, env: &mut Environment) -> Result<RuntimeValue, String> {
    match expr {
        Expr::Program { statements } => {
            let mut result = RuntimeValue::Nil;
            for stmt in statements {
                result = evaluate(stmt, env)?;
            }
            Ok(result)
        }
        Expr::Literal(literal) => match literal {
            LiteralValue::Number(n) => Ok(RuntimeValue::Number(n)),
            LiteralValue::String(msg) => Ok(RuntimeValue::String(msg)),
            _ => todo!("Handle other literals"),
        },
        Expr::Binary { left, op, right } => {
            let l = match evaluate(*left, env)? {
                RuntimeValue::Number(value) => value,
                RuntimeValue::Boolean(cond) => (cond as i64) as f64,
                _ => return Err("Operands for binary expressions must be numbers".to_string()),
            };
            let r = match evaluate(*right, env)? {
                RuntimeValue::Number(value) => value,
                RuntimeValue::Boolean(cond) => (cond as i64) as f64,
                _ => return Err("Operands for binary expressions must be numbers".to_string()),
            };
            match op.kind {
                TokenType::Plus => Ok(RuntimeValue::Number(l + r)),
                TokenType::Minus => Ok(RuntimeValue::Number(l - r)),
                TokenType::Star => Ok(RuntimeValue::Number(l * r)),
                TokenType::Slash => Ok(RuntimeValue::Number(l / r)),
                _ => unreachable!(),
            }
        }
        Expr::Grouping(expr) => evaluate(*expr, env),
        Expr::Binding { name, expr } => {
            let value = evaluate(*expr, env)?;
            env.bind(name, value)?;
            Ok(RuntimeValue::Nil)
        }
        Expr::Variable(name) => env
            .resolve(&name)
            .cloned()
            .ok_or(format!("Name '{name}' is not defined")),
        kind => todo!("Handle other expressions, {:?} not yet implemented", kind),
    }
}
