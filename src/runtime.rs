use std::collections::HashMap;

use crate::ast::Expr;

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)] // until parsing is finished
pub enum RuntimeValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Function { arg: Expr, body: Expr },
    Nil,
}

struct Binding {
    value: RuntimeValue,
    is_constant: bool,
}

pub struct Environment {
    bindings: HashMap<String, Binding>,
}

impl Environment {
    pub fn new() -> Self {
        let mut env = Environment {
            bindings: HashMap::new(),
        };
        env.bind_const(String::from("nil"), RuntimeValue::Nil);
        env.bind_const(String::from("true"), RuntimeValue::Boolean(true));
        env.bind_const(String::from("false"), RuntimeValue::Boolean(false));
        env
    }

    fn bind_const(&mut self, name: String, value: RuntimeValue) {
        self.bindings.insert(
            name,
            Binding {
                value,
                is_constant: true,
            },
        );
    }

    pub fn bind(&mut self, name: String, value: RuntimeValue) -> Result<(), String> {
        if self.bindings.contains_key(&name) && self.bindings[&name].is_constant {
            return Err(format!("Cannot modify variable '{name}'"));
        }
        self.bindings.insert(
            name,
            Binding {
                value,
                is_constant: false,
            },
        );
        Ok(())
    }

    pub fn resolve(&self, name: &str) -> Option<&RuntimeValue> {
        self.bindings.get(name).map(|binding| &binding.value)
    }
}

/// Print to stdout the value contained within the RuntimeValue along with a newline character.
pub fn display(value: &RuntimeValue) {
    match value {
        RuntimeValue::Number(n) => println!("{n}"),
        RuntimeValue::String(msg) => println!("\"{msg}\""),
        RuntimeValue::Boolean(cond) => {
            if *cond {
                println!("true")
            } else {
                println!("false")
            }
        }
        RuntimeValue::Function { arg, body } => println!("function ({:?}) => {:?}", arg, body),
        RuntimeValue::Nil => println!("nil"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_globals() {
        let env = Environment::new();
        assert_eq!(env.resolve("true"), Some(&RuntimeValue::Boolean(true)));
        assert_eq!(env.resolve("nil"), Some(&RuntimeValue::Nil));
    }

    #[test]
    fn test_binding_and_resolving() {
        let mut env = Environment::new();
        let _ = env.bind("x".into(), RuntimeValue::Number(10.0));

        assert_eq!(env.resolve("x"), Some(&RuntimeValue::Number(10.0)));
    }

    #[test]
    fn test_prevent_overwriting_constants() {
        let mut env = Environment::new();
        // Attempt to redefine a global constant
        let result = env.bind("true".into(), RuntimeValue::Boolean(false));

        assert!(result.is_err());
        // Verify the value didn't actually change
        assert_eq!(env.resolve("true"), Some(&RuntimeValue::Boolean(true)));
    }

    #[test]
    fn test_allow_overwriting_variables() {
        let mut mut_env = Environment::new();
        let _ = mut_env.bind("x".into(), RuntimeValue::Number(1.0));
        let _ = mut_env.bind("x".into(), RuntimeValue::Number(2.0)); // Should work

        assert_eq!(mut_env.resolve("x"), Some(&RuntimeValue::Number(2.0)));
    }
}
