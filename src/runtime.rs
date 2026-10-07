use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

use crate::ast::Expr;
use crate::builtins;

#[derive(Clone, Debug)]
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
        function: fn(RuntimeValue, Rc<RefCell<Environment>>) -> Result<RuntimeValue, String>,
    },
    List {
        elements: Vec<RuntimeValue>,
    },
    Nil,
}

impl RuntimeValue {
    /// Converts a RuntimeValue to a bool.
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Number(n) => *n != 0.0,
            Self::String(msg) => !msg.is_empty(),
            Self::Boolean(cond) => *cond,
            Self::Function { .. } => true,
            Self::NativeFunction { .. } => true,
            Self::List { elements } => !elements.is_empty(),
            Self::Nil => false,
        }
    }

    pub fn type_str(&self) -> &str {
        match self {
            Self::Number(_) => "number",
            Self::String(_) => "str",
            Self::Boolean(_) => "bool",
            Self::Function { .. } => "function",
            Self::NativeFunction { .. } => "function",
            Self::List { .. } => "list",
            Self::Nil => "nil",
        }
    }

    pub fn get(&self, i: usize) -> Option<RuntimeValue> {
        match self {
            Self::List { elements } => elements.get(i).cloned(),
            Self::String(elements) => elements
                .chars()
                .nth(i)
                .map(|c| RuntimeValue::String(c.to_string())),
            _ => None,
        }
    }

    pub fn len(&self) -> Result<usize, String> {
        match self {
            Self::List { elements } => Ok(elements.len()),
            Self::String(elements) => Ok(elements.len()),
            _ => Err(format!(
                "Length is only defined for list or string, not {}",
                self.type_str()
            )),
        }
    }

    pub fn is_empty(&self) -> Result<bool, String> {
        match self {
            Self::List { elements } => Ok(elements.is_empty()),
            Self::String(elements) => Ok(elements.is_empty()),
            _ => Err(format!(
                "Length is only defined for list or string, not {}",
                self.type_str()
            )),
        }
    }
}

impl PartialEq for RuntimeValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => a == b,
            (Self::String(a), Self::String(b)) => a == b,
            (Self::Boolean(a), Self::Boolean(b)) => a == b,
            // Functions cannot be compared.
            (Self::Function { .. }, Self::Function { .. }) => false,
            (Self::NativeFunction { .. }, Self::NativeFunction { .. }) => false,
            (Self::List { elements: a }, Self::List { elements: b }) => a == b,
            (Self::Nil, Self::Nil) => true,
            // Two different types are never equal.
            _ => false,
        }
    }
}

impl PartialOrd for RuntimeValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => a.partial_cmp(b),
            (Self::String(a), Self::String(b)) => a.partial_cmp(b),
            (Self::Boolean(a), Self::Boolean(b)) => a.partial_cmp(b),
            // Functions cannot be compared.
            (Self::Function { .. }, Self::Function { .. }) => None,
            (Self::NativeFunction { .. }, Self::NativeFunction { .. }) => None,
            (Self::List { .. }, Self::List { .. }) => None,
            // Allow nil checking.
            (Self::Nil, Self::Nil) => Some(std::cmp::Ordering::Equal),
            // Two different types cannot be compared.
            _ => None,
        }
    }
}

impl Display for RuntimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::String(msg) => write!(f, "\"{msg}\""),
            Self::Boolean(cond) => {
                if *cond {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            }
            Self::Function {
                arg_name,
                body: _,
                closure: _,
            } => write!(f, "<function in {arg_name}>"),
            Self::NativeFunction { name, function: _ } => {
                write!(f, "<native function {name}>")
            }
            Self::List { elements } => {
                write!(f, "[")?;
                for i in 0..elements.len() {
                    write!(f, "{}", elements[i])?;
                    if i < elements.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            Self::Nil => write!(f, "nil"),
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
        Self::new(&[])
    }
}

impl Environment {
    pub fn new(args: &[String]) -> Self {
        let mut env = Environment {
            bindings: HashMap::new(),
            parent: None,
        };

        let elements = args
            .iter()
            .map(|x| RuntimeValue::String(x.clone()))
            .collect();

        env.bind_const("nil", RuntimeValue::Nil);
        env.bind_const("true", RuntimeValue::Boolean(true));
        env.bind_const("false", RuntimeValue::Boolean(false));

        env.bind_const("__args", RuntimeValue::List { elements });
        env.bind_native_fn("__print", builtins::__print);
        env.bind_native_fn("__println", builtins::__println);
        env.bind_native_fn("__read_file", builtins::__read_file);
        env.bind_native_fn("__write_file", builtins::__write_file);

        env.bind_native_fn("__sin", builtins::__sin);
        env.bind_native_fn("__cos", builtins::__cos);
        env.bind_native_fn("__sqrt", builtins::__sqrt);
        env.bind_native_fn("__floor", builtins::__floor);
        env.bind_native_fn("__ceil", builtins::__ceil);

        env.bind_native_fn("__clock", builtins::__clock);

        env.bind_native_fn("__type", builtins::__type);
        env.bind_native_fn("__bool", builtins::__bool);
        env.bind_native_fn("__str", builtins::__str);

        env.bind_native_fn("__map", builtins::__map);
        env.bind_native_fn("__filter", builtins::__filter);
        env.bind_native_fn("__foldl", builtins::__foldl);
        env.bind_native_fn("__foldr", builtins::__foldr);

        env.bind_native_fn("__len", builtins::__len);
        env.bind_native_fn("__get", builtins::__get);
        env.bind_native_fn("__slice", builtins::__slice);
        env.bind_native_fn("__split", builtins::__split);
        env.bind_native_fn("__join", builtins::__join);

        env
    }

    fn bind_native_fn(
        &mut self,
        name: &str,
        function: fn(RuntimeValue, Rc<RefCell<Environment>>) -> Result<RuntimeValue, String>,
    ) {
        let value = RuntimeValue::NativeFunction {
            name: name.into(),
            function,
        };
        self.bind_const(name, value);
    }

    pub fn with_parent(parent: Rc<RefCell<Environment>>) -> Environment {
        Environment {
            bindings: HashMap::new(),
            parent: Some(Rc::clone(&parent)),
        }
    }

    fn bind_const(&mut self, name: &str, value: RuntimeValue) {
        self.bindings.insert(
            name.into(),
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
        let env = Environment::default();
        assert_eq!(env.resolve("true"), Some(RuntimeValue::Boolean(true)));
        assert_eq!(env.resolve("nil"), Some(RuntimeValue::Nil));
    }

    #[test]
    fn test_binding_and_resolving() {
        let mut env = Environment::default();
        let _ = env.bind("x".into(), RuntimeValue::Number(10.0));

        assert_eq!(env.resolve("x"), Some(RuntimeValue::Number(10.0)));
    }

    #[test]
    fn test_prevent_overwriting_constants() {
        let mut env = Environment::default();
        // Attempt to redefine a global constant
        let result = env.bind("true".into(), RuntimeValue::Boolean(false));

        assert!(result.is_err());
        // Verify the value didn't actually change
        assert_eq!(env.resolve("true"), Some(RuntimeValue::Boolean(true)));
    }

    #[test]
    fn test_allow_overwriting_variables() {
        let mut mut_env = Environment::default();
        let _ = mut_env.bind("x".into(), RuntimeValue::Number(1.0));
        let _ = mut_env.assign("x".into(), RuntimeValue::Number(2.0)); // Should work

        assert_eq!(mut_env.resolve("x"), Some(RuntimeValue::Number(2.0)));
    }

    #[test]
    fn test_resolve_parent_env() {
        let env = Rc::new(RefCell::new(Environment::default()));
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
        let env = Rc::new(RefCell::new(Environment::default()));
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
        fn make_native_fn() -> RuntimeValue {
            RuntimeValue::NativeFunction {
                name: "sqrt".into(),
                function: |val, _| Ok(val),
            }
        }

        // Functions cannot be compared
        let f1 = make_native_fn();
        let f2 = make_native_fn();
        assert_ne!(f1, f2);
    }

    #[test]
    fn test_func_equality() {
        fn make_fn() -> RuntimeValue {
            RuntimeValue::Function {
                arg_name: "x".into(),
                body: Expr::Block { statements: vec![] },
                closure: Rc::new(RefCell::new(Environment::default())),
            }
        }

        // Functions cannot be compared
        let f1 = make_fn();
        let f2 = make_fn();
        assert_ne!(f1, f2);
    }
}
