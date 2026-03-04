use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

use crate::ast::Expr;
use crate::builtins;

#[derive(Clone, Debug)]
#[allow(dead_code)] // until parsing is finished
pub enum RuntimeValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Function {
        arg_name: String,
        body: Expr,
        closure: Rc<RefCell<Environment>>,
    },
    NativeFunction {
        name: String,
        function: fn(RuntimeValue) -> Result<RuntimeValue, String>,
    },
    Nil,
}

impl PartialEq for RuntimeValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => a == b,
            (Self::String(a), Self::String(b)) => a == b,
            (Self::Boolean(a), Self::Boolean(b)) => a == b,
            // User-defined functions, check if the argument name, syntax tree and closures match.
            (
                Self::Function {
                    arg_name: name_a,
                    body: body_a,
                    closure: env_a,
                },
                Self::Function {
                    arg_name: name_b,
                    body: body_b,
                    closure: env_b,
                },
            ) => name_a == name_b && body_a == body_b && Rc::ptr_eq(env_a, env_b),
            // For native functions, we check if they share the same identity/name
            (Self::NativeFunction { name: a, .. }, Self::NativeFunction { name: b, .. }) => a == b,
            (Self::Nil, Self::Nil) => true,
            _ => false,
        }
    }
}

impl Display for RuntimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeValue::Number(n) => write!(f, "{n}"),
            RuntimeValue::String(msg) => write!(f, "\"{msg}\""),
            RuntimeValue::Boolean(cond) => {
                if *cond {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            }
            RuntimeValue::Function {
                arg_name,
                body: _,
                closure: _,
            } => write!(f, "<function in {arg_name}>"),
            RuntimeValue::NativeFunction { name, function: _ } => {
                write!(f, "<native function {name}>")
            }
            RuntimeValue::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Binding {
    value: RuntimeValue,
    is_constant: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Environment {
    bindings: HashMap<String, Binding>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        let mut env = Environment {
            bindings: HashMap::new(),
            parent: None,
        };
        env.bind_const(String::from("nil"), RuntimeValue::Nil);
        env.bind_const(String::from("true"), RuntimeValue::Boolean(true));
        env.bind_const(String::from("false"), RuntimeValue::Boolean(false));

        env.bind_native_fn("sin", builtins::sin);
        env.bind_native_fn("cos", builtins::cos);
        env.bind_native_fn("sqrt", builtins::sqrt);
        env.bind_native_fn("clock", builtins::clock);
        env.bind_native_fn("str", builtins::str);
        env.bind_native_fn("print", builtins::print);
        env.bind_native_fn("println", builtins::println);

        env
    }

    fn bind_native_fn(
        &mut self,
        name: &str,
        function: fn(RuntimeValue) -> Result<RuntimeValue, String>,
    ) {
        let value = RuntimeValue::NativeFunction {
            name: name.into(),
            function,
        };
        self.bind_const(name.into(), value);
    }

    pub fn with_parent(parent: Rc<RefCell<Environment>>) -> Environment {
        Environment {
            bindings: HashMap::new(),
            parent: Some(Rc::clone(&parent)),
        }
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
        if self.bindings.contains_key(&name) {
            return Err(format!("Cannot redeclare variable '{name}'"));
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

    pub fn assign(&mut self, name: String, value: RuntimeValue) -> Result<(), String> {
        if let Some(binding) = self.bindings.get(&name) {
            if binding.is_constant {
                Err(format!("Cannot modify constant variable '{name}'"))
            } else {
                self.bindings.insert(
                    name,
                    Binding {
                        value,
                        is_constant: false,
                    },
                );
                Ok(())
            }
        } else if let Some(parent) = &self.parent {
            parent.borrow_mut().assign(name, value)
        } else {
            Err(format!("Name '{name}' is not defined"))
        }
    }

    pub fn resolve(&self, name: &str) -> Option<RuntimeValue> {
        if let Some(binding) = self.bindings.get(name) {
            return Some(binding.value.clone());
        }

        // use &self.parent to avoid moving the Rc out of the struct
        if let Some(parent) = &self.parent {
            // recursively call resolve on the parent
            parent.borrow().resolve(name)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_globals() {
        let env = Environment::new();
        assert_eq!(env.resolve("true"), Some(RuntimeValue::Boolean(true)));
        assert_eq!(env.resolve("nil"), Some(RuntimeValue::Nil));
    }

    #[test]
    fn test_binding_and_resolving() {
        let mut env = Environment::new();
        let _ = env.bind("x".into(), RuntimeValue::Number(10.0));

        assert_eq!(env.resolve("x"), Some(RuntimeValue::Number(10.0)));
    }

    #[test]
    fn test_prevent_overwriting_constants() {
        let mut env = Environment::new();
        // Attempt to redefine a global constant
        let result = env.bind("true".into(), RuntimeValue::Boolean(false));

        assert!(result.is_err());
        // Verify the value didn't actually change
        assert_eq!(env.resolve("true"), Some(RuntimeValue::Boolean(true)));
    }

    #[test]
    fn test_allow_overwriting_variables() {
        let mut mut_env = Environment::new();
        let _ = mut_env.bind("x".into(), RuntimeValue::Number(1.0));
        let _ = mut_env.assign("x".into(), RuntimeValue::Number(2.0)); // Should work

        assert_eq!(mut_env.resolve("x"), Some(RuntimeValue::Number(2.0)));
    }

    #[test]
    fn test_resolve_parent_env() {
        let env = Rc::new(RefCell::new(Environment::new()));
        env.borrow_mut()
            .bind("x".into(), RuntimeValue::Number(1.0))
            .expect("Binding should not fail");

        let local_env = Environment::with_parent(Rc::clone(&env));

        // Both envs should resolve x
        assert_eq!(env.borrow().resolve("x"), Some(RuntimeValue::Number(1.0)));
        assert_eq!(local_env.resolve("x"), Some(RuntimeValue::Number(1.0)));
    }

    #[test]
    fn test_shadowing_env() {
        let env = Rc::new(RefCell::new(Environment::new()));
        env.borrow_mut()
            .bind("x".into(), RuntimeValue::Number(1.0))
            .expect("Binding should not fail");

        let mut local_env = Environment::with_parent(Rc::clone(&env));
        local_env
            .bind("x".into(), RuntimeValue::Number(2.0))
            .expect("Binding should not fail");

        // Outer env should not be affected
        assert_eq!(env.borrow().resolve("x"), Some(RuntimeValue::Number(1.0)));

        // Inner env variable shadows the outer scope variable
        assert_eq!(local_env.resolve("x"), Some(RuntimeValue::Number(2.0)));
    }

    #[test]
    fn test_native_func_equality() {
        let f1 = RuntimeValue::NativeFunction {
            name: "sqrt".into(),
            function: |val| Ok(val),
        };
        let f2 = RuntimeValue::NativeFunction {
            name: "sqrt".into(),
            function: |val| Ok(val),
        };

        // Uses the name "sqrt" to check equality
        assert_eq!(f1, f2);
    }
}
