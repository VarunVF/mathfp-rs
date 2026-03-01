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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Token, TokenType};

    // Helper to create a dummy token for operators
    fn op_token(kind: TokenType) -> Token {
        Token {
            kind,
            lexeme: String::new(),
            line: 1,
            column: 1,
        }
    }

    #[test]
    fn test_literals() {
        let mut env = Environment::new();

        let num_res = evaluate(Expr::Literal(LiteralValue::Number(42.0)), &mut env).unwrap();
        assert_eq!(num_res, RuntimeValue::Number(42.0));

        let str_res = evaluate(
            Expr::Literal(LiteralValue::String("MathFP".into())),
            &mut env,
        )
        .unwrap();
        assert_eq!(str_res, RuntimeValue::String("MathFP".into()));
    }

    #[test]
    fn test_binary_arithmetic() {
        let mut env = Environment::new();

        // 10 + 5
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(LiteralValue::Number(10.0))),
            op: op_token(TokenType::Plus),
            right: Box::new(Expr::Literal(LiteralValue::Number(5.0))),
        };
        assert_eq!(
            evaluate(expr, &mut env).unwrap(),
            RuntimeValue::Number(15.0)
        );
    }

    #[test]
    fn test_boolean_to_number_coercion() {
        let mut env = Environment::new();

        // true + 1 (should be 1.0 + 1.0 = 2.0)
        let expr = Expr::Binary {
            left: Box::new(Expr::Variable("true".into())),
            op: op_token(TokenType::Plus),
            right: Box::new(Expr::Literal(LiteralValue::Number(1.0))),
        };
        assert_eq!(evaluate(expr, &mut env).unwrap(), RuntimeValue::Number(2.0));
    }

    #[test]
    fn test_bindings_and_variables() {
        let mut env = Environment::new();

        // x := 100
        let bind_expr = Expr::Binding {
            name: "x".into(),
            expr: Box::new(Expr::Literal(LiteralValue::Number(100.0))),
        };
        evaluate(bind_expr, &mut env).unwrap();

        // resolve x
        let var_expr = Expr::Variable("x".into());
        assert_eq!(
            evaluate(var_expr, &mut env).unwrap(),
            RuntimeValue::Number(100.0)
        );
    }

    #[test]
    #[should_panic(expected = "Cannot modify variable")]
    fn test_constant_protection() {
        let mut env = Environment::new(); // Environment::new() adds "true" as a constant

        // true := 5 (should fail)
        let expr = Expr::Binding {
            name: "true".into(),
            expr: Box::new(Expr::Literal(LiteralValue::Number(5.0))),
        };

        evaluate(expr, &mut env).unwrap();
    }

    #[test]
    fn test_unresolved_variable() {
        let mut env = Environment::new();
        let expr = Expr::Variable("x".into());

        let result = evaluate(expr, &mut env);
        assert_eq!(result.unwrap_err(), "Name 'x' is not defined");
    }

    #[test]
    fn test_grouping() {
        let mut env = Environment::new();
        // (10)
        let expr = Expr::Grouping(Box::new(Expr::Literal(LiteralValue::Number(10.0))));
        assert_eq!(
            evaluate(expr, &mut env).unwrap(),
            RuntimeValue::Number(10.0)
        );
    }

    #[test]
    fn test_program_sequence() {
        let mut env = Environment::new();
        // a := 1; a + 2;
        let prog = Expr::Program {
            statements: vec![
                Expr::Binding {
                    name: "a".into(),
                    expr: Box::new(Expr::Literal(LiteralValue::Number(1.0))),
                },
                Expr::Binary {
                    left: Box::new(Expr::Variable("a".into())),
                    op: op_token(TokenType::Plus),
                    right: Box::new(Expr::Literal(LiteralValue::Number(2.0))),
                },
            ],
        };
        // Program should return the result of the last statement (3.0)
        assert_eq!(evaluate(prog, &mut env).unwrap(), RuntimeValue::Number(3.0));
    }
}
