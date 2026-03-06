use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::{Expr, LiteralValue};
use crate::runtime::{Environment, RuntimeValue};
use crate::token::{Token, TokenType};

fn make_unsupported_binary_expr_err(
    left: &RuntimeValue,
    right: &RuntimeValue,
    op: &Token,
) -> Result<RuntimeValue, String> {
    Err(format!(
        "Unsupported operands for '{}': {left}, {right}",
        op.lexeme
    ))
}

pub fn evaluate(expr: Expr, env: Rc<RefCell<Environment>>) -> Result<RuntimeValue, String> {
    match expr {
        Expr::Program { statements } => {
            let mut result = RuntimeValue::Nil;
            for stmt in statements {
                result = evaluate(stmt, Rc::clone(&env))?;
            }
            Ok(result)
        }
        Expr::Literal(literal) => match literal {
            LiteralValue::Number(n) => Ok(RuntimeValue::Number(n)),
            LiteralValue::String(msg) => Ok(RuntimeValue::String(msg)),
            LiteralValue::Nil => Ok(RuntimeValue::Nil),
            LiteralValue::Boolean(cond) => Ok(RuntimeValue::Boolean(cond)),
        },
        Expr::Binary { left, op, right } => {
            let left = evaluate(*left, Rc::clone(&env))?;
            let right = evaluate(*right, Rc::clone(&env))?;
            match (&left, &right) {
                // For numbers
                (RuntimeValue::Number(left), RuntimeValue::Number(right)) => match op.kind {
                    TokenType::Plus => Ok(RuntimeValue::Number(left + right)),
                    TokenType::Minus => Ok(RuntimeValue::Number(left - right)),
                    TokenType::Star => Ok(RuntimeValue::Number(left * right)),
                    TokenType::Slash => Ok(RuntimeValue::Number(left / right)),
                    TokenType::Less => Ok(RuntimeValue::Boolean(left < right)),
                    TokenType::LessEqual => Ok(RuntimeValue::Boolean(left <= right)),
                    TokenType::Greater => Ok(RuntimeValue::Boolean(left > right)),
                    TokenType::GreaterEqual => Ok(RuntimeValue::Boolean(left >= right)),
                    TokenType::BangEqual => Ok(RuntimeValue::Boolean(left != right)),
                    TokenType::EqualEqual => Ok(RuntimeValue::Boolean(left == right)),
                    _ => unreachable!("There should be no other binary operators"),
                },
                (_, _) => match op.kind {
                    // Plus also defined for String.
                    TokenType::Plus => match (left, right) {
                        (RuntimeValue::String(left), RuntimeValue::String(right)) => {
                            Ok(RuntimeValue::String(format!("{left}{right}")))
                        }
                        (left, right) => make_unsupported_binary_expr_err(&left, &right, &op),
                    },
                    // Minus, Star, Slash not defined for other types.
                    TokenType::Minus | TokenType::Star | TokenType::Slash => {
                        make_unsupported_binary_expr_err(&left, &right, &op)
                    }
                    TokenType::Less => Ok(RuntimeValue::Boolean(left < right)),
                    TokenType::LessEqual => Ok(RuntimeValue::Boolean(left <= right)),
                    TokenType::Greater => Ok(RuntimeValue::Boolean(left > right)),
                    TokenType::GreaterEqual => Ok(RuntimeValue::Boolean(left >= right)),
                    TokenType::BangEqual => Ok(RuntimeValue::Boolean(left != right)),
                    TokenType::EqualEqual => Ok(RuntimeValue::Boolean(left == right)),
                    _ => unreachable!("There should be no other binary operators"),
                },
            }
        }
        Expr::Unary { op, right } => {
            let r = evaluate(*right, env)?;
            match (op.kind, r.clone()) {
                (TokenType::Minus, RuntimeValue::Number(n)) => Ok(RuntimeValue::Number(-n)),
                (TokenType::Minus, _) => Err("Operand for unary '-' must be a number".to_string()),
                (TokenType::Bang, RuntimeValue::Boolean(cond)) => Ok(RuntimeValue::Boolean(!cond)),
                (TokenType::Bang, _) => Ok(RuntimeValue::Boolean(!is_truthy(&r))),
                _ => unreachable!("There should only be '-' or '!' unary operators"),
            }
        }
        Expr::Grouping(expr) => evaluate(*expr, env),
        Expr::Binding { name, expr } => {
            let value = evaluate(*expr, Rc::clone(&env))?;
            env.borrow_mut().bind(name, value.clone())?;
            Ok(value)
        }
        Expr::Assign { name, expr } => {
            let value = evaluate(*expr, Rc::clone(&env))?;
            env.borrow_mut().assign(name, value.clone())?;
            Ok(value)
        }
        Expr::Variable(name) => env
            .borrow()
            .resolve(&name)
            .ok_or(format!("Name '{name}' is not defined")),
        Expr::If {
            cond_expr,
            then_expr,
            else_expr,
        } => {
            // Lazy evaluation of branches
            if is_truthy(&evaluate(*cond_expr, Rc::clone(&env))?) {
                evaluate(*then_expr, env)
            } else {
                evaluate(*else_expr, env)
            }
        }
        Expr::FunctionDef { param, body } => Ok(RuntimeValue::Function {
            arg_name: param,
            body: *body,
            closure: Rc::clone(&env),
        }),
        Expr::FunctionBody { statements } => {
            // An empty function body should return nil
            let mut res = RuntimeValue::Nil;
            for stmt in statements {
                // If encountered error, stop and return immediately
                res = evaluate(stmt, Rc::clone(&env))?;
            }
            Ok(res)
        }
        Expr::FunctionCall { func, arg } => {
            let function = evaluate(*func, Rc::clone(&env))?;

            match function {
                RuntimeValue::Function {
                    arg_name,
                    body,
                    closure,
                } => {
                    let arg_value = evaluate(*arg, Rc::clone(&env))?;

                    // The parent of the new scope is the closure
                    let local_env = Rc::new(RefCell::new(Environment::with_parent(closure)));
                    local_env.borrow_mut().bind(arg_name, arg_value)?;
                    evaluate(body, local_env)
                }
                RuntimeValue::NativeFunction { name: _, function } => {
                    let arg_value = evaluate(*arg, Rc::clone(&env))?;
                    Ok(function(arg_value)?)
                }
                _ => Err("Only functions are callable".to_string()),
            }
        }
        Expr::Empty => unreachable!("The program should never contain Empty expressions"),
    }
}

/// Coerces a RuntimeValue to a bool.
fn is_truthy(value: &RuntimeValue) -> bool {
    match value {
        RuntimeValue::Number(n) => *n != 0.0,
        RuntimeValue::String(msg) => !msg.is_empty(),
        RuntimeValue::Boolean(cond) => *cond,
        RuntimeValue::Function { .. } => true,
        RuntimeValue::NativeFunction { .. } => true,
        RuntimeValue::Nil => false,
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
        let env = Rc::new(RefCell::new(Environment::new()));

        let num_res = evaluate(Expr::Literal(LiteralValue::Number(42.0)), Rc::clone(&env)).unwrap();
        assert_eq!(num_res, RuntimeValue::Number(42.0));

        let str_res = evaluate(
            Expr::Literal(LiteralValue::String("MathFP".into())),
            Rc::clone(&env),
        )
        .unwrap();
        assert_eq!(str_res, RuntimeValue::String("MathFP".into()));
    }

    #[test]
    fn test_binary_arithmetic() {
        let env = Rc::new(RefCell::new(Environment::new()));

        // 10 + 5
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(LiteralValue::Number(10.0))),
            op: op_token(TokenType::Plus),
            right: Box::new(Expr::Literal(LiteralValue::Number(5.0))),
        };
        assert_eq!(
            evaluate(expr, Rc::clone(&env)).unwrap(),
            RuntimeValue::Number(15.0)
        );
    }

    #[test]
    fn test_bindings_and_variables() {
        let env = Rc::new(RefCell::new(Environment::new()));

        // x := 100
        let bind_expr = Expr::Binding {
            name: "x".into(),
            expr: Box::new(Expr::Literal(LiteralValue::Number(100.0))),
        };
        evaluate(bind_expr, Rc::clone(&env)).unwrap();

        // resolve x
        let var_expr = Expr::Variable("x".into());
        assert_eq!(
            evaluate(var_expr, Rc::clone(&env)).unwrap(),
            RuntimeValue::Number(100.0)
        );
    }

    #[test]
    #[should_panic(expected = "Cannot redeclare variable")]
    fn test_constant_protection() {
        let env = Rc::new(RefCell::new(Environment::new())); // Environment::new() adds "true" as a constant

        // true := 5 (should fail)
        let expr = Expr::Binding {
            name: "true".into(),
            expr: Box::new(Expr::Literal(LiteralValue::Number(5.0))),
        };

        evaluate(expr, Rc::clone(&env)).unwrap();
    }

    #[test]
    fn test_unresolved_variable() {
        let env = Rc::new(RefCell::new(Environment::new()));
        let expr = Expr::Variable("x".into());

        let result = evaluate(expr, Rc::clone(&env));
        assert_eq!(result.unwrap_err(), "Name 'x' is not defined");
    }

    #[test]
    fn test_grouping() {
        let env = Rc::new(RefCell::new(Environment::new()));
        // (10)
        let expr = Expr::Grouping(Box::new(Expr::Literal(LiteralValue::Number(10.0))));
        assert_eq!(
            evaluate(expr, Rc::clone(&env)).unwrap(),
            RuntimeValue::Number(10.0)
        );
    }

    #[test]
    fn test_if_basic_branching() {
        let env = Rc::new(RefCell::new(Environment::new()));

        // if true then 10 else 20
        let expr = Expr::If {
            cond_expr: Box::new(Expr::Variable("true".into())),
            then_expr: Box::new(Expr::Literal(LiteralValue::Number(10.0))),
            else_expr: Box::new(Expr::Literal(LiteralValue::Number(20.0))),
        };
        assert_eq!(
            evaluate(expr, Rc::clone(&env)).unwrap(),
            RuntimeValue::Number(10.0)
        );

        // if false then 10 else 20
        let expr_false = Expr::If {
            cond_expr: Box::new(Expr::Variable("false".into())),
            then_expr: Box::new(Expr::Literal(LiteralValue::Number(10.0))),
            else_expr: Box::new(Expr::Literal(LiteralValue::Number(20.0))),
        };
        assert_eq!(
            evaluate(expr_false, Rc::clone(&env)).unwrap(),
            RuntimeValue::Number(20.0)
        );
    }

    #[test]
    fn test_program_sequence() {
        let env = Rc::new(RefCell::new(Environment::new()));
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
        assert_eq!(
            evaluate(prog, Rc::clone(&env)).unwrap(),
            RuntimeValue::Number(3.0)
        );
    }
}
