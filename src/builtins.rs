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

pub fn bool(value: RuntimeValue) -> Result<RuntimeValue, String> {
    // Convert to a boolean runtime value
    Ok(RuntimeValue::Boolean(value.is_truthy()))
}

pub fn str(value: RuntimeValue) -> Result<RuntimeValue, String> {
    // Convert to a string runtime value
    Ok(RuntimeValue::String(value.to_string()))
}

pub fn print(value: RuntimeValue) -> Result<RuntimeValue, String> {
    print!("{value}");
    Ok(RuntimeValue::Nil)
}

pub fn println(value: RuntimeValue) -> Result<RuntimeValue, String> {
    println!("{value}");
    Ok(RuntimeValue::Nil)
}
