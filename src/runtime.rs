use std::collections::HashMap;

use crate::ast::Expr;

#[derive(Clone, Debug)]
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
