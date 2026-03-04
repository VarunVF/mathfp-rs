use std::time::{SystemTime, UNIX_EPOCH};

use crate::runtime::RuntimeValue;

pub fn sin(value: RuntimeValue) -> Result<RuntimeValue, String> {
    match value {
        RuntimeValue::Number(n) => Ok(RuntimeValue::Number(n.sin())),
        _ => Err("sin() expects a number".into()),
    }
}

pub fn cos(value: RuntimeValue) -> Result<RuntimeValue, String> {
    match value {
        RuntimeValue::Number(n) => Ok(RuntimeValue::Number(n.cos())),
        _ => Err("cos() expects a number".into()),
    }
}

pub fn sqrt(value: RuntimeValue) -> Result<RuntimeValue, String> {
    match value {
        RuntimeValue::Number(n) => Ok(RuntimeValue::Number(n.sqrt())),
        _ => Err("sqrt() expects a number".into()),
    }
}

// This function takes no argument (Nil).
pub fn clock(value: RuntimeValue) -> Result<RuntimeValue, String> {
    match value {
        RuntimeValue::Nil => {
            let start = SystemTime::now();
            let time_since_epoch = start
                .duration_since(UNIX_EPOCH)
                .map_err(|_| "Time went backwards")?;

            // Return time in seconds as a float
            Ok(RuntimeValue::Number(time_since_epoch.as_secs_f64()))
        }
        _ => Err("clock() takes no argument, pass `nil` instead".into()),
    }
}

pub fn str(value: RuntimeValue) -> Result<RuntimeValue, String> {
    match value {
        RuntimeValue::Number(n) => Ok(RuntimeValue::String(n.to_string())),
        RuntimeValue::String(msg) => Ok(RuntimeValue::String(msg)),
        RuntimeValue::Boolean(cond) => {
            if cond {
                Ok(RuntimeValue::String("true".to_string()))
            } else {
                Ok(RuntimeValue::String("false".to_string()))
            }
        }
        RuntimeValue::Function {
            arg_name,
            body: _,
            closure: _,
        } => Ok(RuntimeValue::String(format!("<function in {arg_name}>"))),
        RuntimeValue::NativeFunction { name, function: _ } => {
            Ok(RuntimeValue::String(format!("<native function {name}>")))
        }
        RuntimeValue::Nil => Ok(RuntimeValue::String("nil".to_string())),
    }
}

pub fn print(value: RuntimeValue) -> Result<RuntimeValue, String> {
    match value {
        RuntimeValue::Number(n) => print!("{}", n),
        RuntimeValue::String(msg) => print!("{}", msg),
        RuntimeValue::Boolean(cond) => {
            if cond {
                print!("true")
            } else {
                print!("false")
            }
        }
        RuntimeValue::Function {
            arg_name,
            body: _,
            closure: _,
        } => print!("<function in {arg_name}>"),
        RuntimeValue::NativeFunction { name, function: _ } => print!("<native function {name}>"),
        RuntimeValue::Nil => print!("nil"),
    }

    Ok(RuntimeValue::Nil)
}

pub fn println(value: RuntimeValue) -> Result<RuntimeValue, String> {
    print(value)?;
    println!();
    Ok(RuntimeValue::Nil)
}
