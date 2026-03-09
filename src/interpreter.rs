use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::{Expr, LiteralValue};
use crate::runtime::{Environment, RuntimeValue};
use crate::token::{Token, TokenType};

pub struct Interpreter {
    globals: Rc<RefCell<Environment>>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            globals: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn interpret(&self, expr: Expr) -> Result<RuntimeValue, String> {
        Self::execute(expr, Rc::clone(&self.globals))
    }

    pub fn value_of(&self, name: &str) -> Option<RuntimeValue> {
        self.globals.borrow().resolve(name)
    }

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

    fn execute(expr: Expr, env: Rc<RefCell<Environment>>) -> Result<RuntimeValue, String> {
        match expr {
            Expr::Program { statements } => Self::execute_program(statements, Rc::clone(&env)),
            Expr::Literal(literal) => Self::execute_literal(literal),
            Expr::Binary { left, op, right } => {
                Self::execute_binary(*left, op, *right, Rc::clone(&env))
            }
            Expr::Unary { op, right } => Self::execute_unary(op, *right, Rc::clone(&env)),
            Expr::Grouping(expr) => Self::execute(*expr, Rc::clone(&env)),
            Expr::Binding { name, expr } => Self::execute_binding(name, *expr, Rc::clone(&env)),
            Expr::Assign { name, expr } => Self::execute_assign(name, *expr, Rc::clone(&env)),
            Expr::Variable(name) => Self::execute_variable(name, Rc::clone(&env)),
            Expr::If {
                cond_expr,
                then_expr,
                else_expr,
            } => Self::execute_if(*cond_expr, *then_expr, *else_expr, Rc::clone(&env)),
            Expr::FunctionDef { param, body } => {
                Self::execute_function_def(param, *body, Rc::clone(&env))
            }
            Expr::FunctionBody { statements } => {
                Self::execute_function_body(statements, Rc::clone(&env))
            }
            Expr::FunctionCall { func, arg } => {
                Self::execute_function_call(*func, *arg, Rc::clone(&env))
            }
            Expr::Empty => unreachable!("The program should never contain Empty expressions"),
        }
    }

    fn execute_program(
        statements: Vec<Expr>,
        env: Rc<RefCell<Environment>>,
    ) -> Result<RuntimeValue, String> {
        let mut result = RuntimeValue::Nil;
        for stmt in statements {
            result = Self::execute(stmt, Rc::clone(&env))?;
        }
        Ok(result)
    }

    fn execute_literal(literal: LiteralValue) -> Result<RuntimeValue, String> {
        match literal {
            LiteralValue::Number(n) => Ok(RuntimeValue::Number(n)),
            LiteralValue::String(msg) => Ok(RuntimeValue::String(msg)),
            LiteralValue::Nil => Ok(RuntimeValue::Nil),
            LiteralValue::Boolean(cond) => Ok(RuntimeValue::Boolean(cond)),
        }
    }

    fn execute_binary(
        left: Expr,
        op: Token,
        right: Expr,
        env: Rc<RefCell<Environment>>,
    ) -> Result<RuntimeValue, String> {
        let left = Self::execute(left, Rc::clone(&env))?;
        let right = Self::execute(right, Rc::clone(&env))?;
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
                    (left, right) => Self::make_unsupported_binary_expr_err(&left, &right, &op),
                },
                // Minus, Star, Slash not defined for other types.
                TokenType::Minus | TokenType::Star | TokenType::Slash => {
                    Self::make_unsupported_binary_expr_err(&left, &right, &op)
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

    fn execute_unary(
        op: Token,
        right: Expr,
        env: Rc<RefCell<Environment>>,
    ) -> Result<RuntimeValue, String> {
        let r = Self::execute(right, env)?;
        match (op.kind, r.clone()) {
            (TokenType::Minus, RuntimeValue::Number(n)) => Ok(RuntimeValue::Number(-n)),
            (TokenType::Minus, _) => Err("Operand for unary '-' must be a number".to_string()),
            (TokenType::Bang, RuntimeValue::Boolean(cond)) => Ok(RuntimeValue::Boolean(!cond)),
            (TokenType::Bang, _) => Ok(RuntimeValue::Boolean(!Self::is_truthy(&r))),
            _ => unreachable!("There should only be '-' or '!' unary operators"),
        }
    }

    fn execute_binding(
        name: String,
        expr: Expr,
        env: Rc<RefCell<Environment>>,
    ) -> Result<RuntimeValue, String> {
        let value = Self::execute(expr, Rc::clone(&env))?;
        env.borrow_mut().bind(name, value.clone())?;
        Ok(value)
    }

    fn execute_assign(
        name: String,
        expr: Expr,
        env: Rc<RefCell<Environment>>,
    ) -> Result<RuntimeValue, String> {
        let value = Self::execute(expr, Rc::clone(&env))?;
        env.borrow_mut().assign(name, value.clone())?;
        Ok(value)
    }

    fn execute_variable(
        name: String,
        env: Rc<RefCell<Environment>>,
    ) -> Result<RuntimeValue, String> {
        env.borrow()
            .resolve(&name)
            .ok_or(format!("Name '{name}' is not defined"))
    }

    fn execute_if(
        cond_expr: Expr,
        then_expr: Expr,
        else_expr: Expr,
        env: Rc<RefCell<Environment>>,
    ) -> Result<RuntimeValue, String> {
        // Lazy evaluation of branches
        if Self::is_truthy(&Self::execute(cond_expr, Rc::clone(&env))?) {
            Self::execute(then_expr, env)
        } else {
            Self::execute(else_expr, env)
        }
    }

    fn execute_function_def(
        param: String,
        body: Expr,
        env: Rc<RefCell<Environment>>,
    ) -> Result<RuntimeValue, String> {
        Ok(RuntimeValue::Function {
            arg_name: param,
            body,
            closure: Rc::clone(&env),
        })
    }

    fn execute_function_body(
        statements: Vec<Expr>,
        env: Rc<RefCell<Environment>>,
    ) -> Result<RuntimeValue, String> {
        // An empty function body should return nil
        let mut res = RuntimeValue::Nil;
        for stmt in statements {
            // If encountered error, stop and return immediately
            res = Self::execute(stmt, Rc::clone(&env))?;
        }
        Ok(res)
    }

    fn execute_function_call(
        func: Expr,
        arg: Expr,
        env: Rc<RefCell<Environment>>,
    ) -> Result<RuntimeValue, String> {
        let function = Self::execute(func, Rc::clone(&env))?;

        match function {
            RuntimeValue::Function {
                arg_name,
                body,
                closure,
            } => {
                let arg_value = Self::execute(arg, Rc::clone(&env))?;

                // The parent of the new scope is the closure
                let local_env = Rc::new(RefCell::new(Environment::with_parent(closure)));
                local_env.borrow_mut().bind(arg_name, arg_value)?;
                Self::execute(body, local_env)
            }
            RuntimeValue::NativeFunction { name: _, function } => {
                let arg_value = Self::execute(arg, Rc::clone(&env))?;
                Ok(function(arg_value)?)
            }
            _ => Err("Only functions are callable".to_string()),
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
        let interpreter = Interpreter::new();
        let num_res = interpreter
            .interpret(Expr::Literal(LiteralValue::Number(42.0)))
            .unwrap();
        assert_eq!(num_res, RuntimeValue::Number(42.0));

        let str_res = interpreter
            .interpret(Expr::Literal(LiteralValue::String("MathFP".into())))
            .unwrap();
        assert_eq!(str_res, RuntimeValue::String("MathFP".into()));
    }

    #[test]
    fn test_binary_arithmetic() {
        let interpreter = Interpreter::new();

        // 10 + 5
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(LiteralValue::Number(10.0))),
            op: op_token(TokenType::Plus),
            right: Box::new(Expr::Literal(LiteralValue::Number(5.0))),
        };
        assert_eq!(
            interpreter.interpret(expr).unwrap(),
            RuntimeValue::Number(15.0)
        );
    }

    #[test]
    fn test_bindings_and_variables() {
        let interpreter = Interpreter::new();

        // x := 100
        let bind_expr = Expr::Binding {
            name: "x".into(),
            expr: Box::new(Expr::Literal(LiteralValue::Number(100.0))),
        };
        interpreter.interpret(bind_expr).unwrap();

        // resolve x
        let var_expr = Expr::Variable("x".into());
        assert_eq!(
            interpreter.interpret(var_expr).unwrap(),
            RuntimeValue::Number(100.0)
        );
    }

    #[test]
    #[should_panic(expected = "Cannot redeclare variable")]
    fn test_constant_protection() {
        let interpreter = Interpreter::new(); // Adds "true" as a constant

        // true := 5 (should fail)
        let expr = Expr::Binding {
            name: "true".into(),
            expr: Box::new(Expr::Literal(LiteralValue::Number(5.0))),
        };

        interpreter.interpret(expr).unwrap();
    }

    #[test]
    fn test_unresolved_variable() {
        let interpreter = Interpreter::new();

        let result = interpreter.interpret(Expr::Variable("x".into()));
        assert_eq!(result.unwrap_err(), "Name 'x' is not defined");
    }

    #[test]
    fn test_grouping() {
        let interpreter = Interpreter::new();

        // (10)
        let expr = Expr::Grouping(Box::new(Expr::Literal(LiteralValue::Number(10.0))));
        assert_eq!(
            interpreter.interpret(expr).unwrap(),
            RuntimeValue::Number(10.0)
        );
    }

    #[test]
    fn test_if_basic_branching() {
        let interpreter = Interpreter::new();

        // if true then 10 else 20
        let expr = Expr::If {
            cond_expr: Box::new(Expr::Variable("true".into())),
            then_expr: Box::new(Expr::Literal(LiteralValue::Number(10.0))),
            else_expr: Box::new(Expr::Literal(LiteralValue::Number(20.0))),
        };
        assert_eq!(
            interpreter.interpret(expr).unwrap(),
            RuntimeValue::Number(10.0)
        );

        // if false then 10 else 20
        let expr_false = Expr::If {
            cond_expr: Box::new(Expr::Variable("false".into())),
            then_expr: Box::new(Expr::Literal(LiteralValue::Number(10.0))),
            else_expr: Box::new(Expr::Literal(LiteralValue::Number(20.0))),
        };
        assert_eq!(
            interpreter.interpret(expr_false).unwrap(),
            RuntimeValue::Number(20.0)
        );
    }

    #[test]
    fn test_program_sequence() {
        let interpreter = Interpreter::new();

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
            interpreter.interpret(prog).unwrap(),
            RuntimeValue::Number(3.0)
        );
    }
}
