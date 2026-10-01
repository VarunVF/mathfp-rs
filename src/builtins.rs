use std::{
    cell::RefCell,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    interpreter::Interpreter,
    runtime::{Environment, RuntimeValue},
};

pub fn __sin(value: RuntimeValue, _env: Rc<RefCell<Environment>>) -> Result<RuntimeValue, String> {
    match value {
        RuntimeValue::Number(n) => Ok(RuntimeValue::Number(n.sin())),
        _ => Err("sin() expects a number".into()),
    }
}

pub fn __cos(value: RuntimeValue, _env: Rc<RefCell<Environment>>) -> Result<RuntimeValue, String> {
    match value {
        RuntimeValue::Number(n) => Ok(RuntimeValue::Number(n.cos())),
        _ => Err("cos() expects a number".into()),
    }
}

pub fn __sqrt(value: RuntimeValue, _env: Rc<RefCell<Environment>>) -> Result<RuntimeValue, String> {
    match value {
        RuntimeValue::Number(n) => Ok(RuntimeValue::Number(n.sqrt())),
        _ => Err("sqrt() expects a number".into()),
    }
}

// This function takes no argument (Nil).
pub fn __clock(
    value: RuntimeValue,
    _env: Rc<RefCell<Environment>>,
) -> Result<RuntimeValue, String> {
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

pub fn __bool(value: RuntimeValue, _env: Rc<RefCell<Environment>>) -> Result<RuntimeValue, String> {
    // Convert to a boolean runtime value
    Ok(RuntimeValue::Boolean(value.is_truthy()))
}

pub fn __str(value: RuntimeValue, _env: Rc<RefCell<Environment>>) -> Result<RuntimeValue, String> {
    // Convert to a string runtime value
    Ok(RuntimeValue::String(value.to_string()))
}

pub fn __print(
    value: RuntimeValue,
    _env: Rc<RefCell<Environment>>,
) -> Result<RuntimeValue, String> {
    match value {
        RuntimeValue::String(contents) => print!("{contents}"),
        _ => print!("{value}"),
    }
    Ok(RuntimeValue::Nil)
}

pub fn __println(
    value: RuntimeValue,
    _env: Rc<RefCell<Environment>>,
) -> Result<RuntimeValue, String> {
    match value {
        RuntimeValue::String(contents) => println!("{contents}"),
        _ => println!("{value}"),
    }
    Ok(RuntimeValue::Nil)
}

pub fn __map(value: RuntimeValue, env: Rc<RefCell<Environment>>) -> Result<RuntimeValue, String> {
    if let RuntimeValue::List { elements: map_args } = value
        && let Some(RuntimeValue::Function { .. }) = map_args.first()
        && let Some(RuntimeValue::List {
            elements: original_items,
        }) = map_args.get(1)
    {
        let mut mapped = Vec::new();
        for item in original_items {
            let new_value =
                Interpreter::interpret_function(map_args[0].clone(), item.clone(), Rc::clone(&env));
            mapped.push(new_value?);
        }
        Ok(RuntimeValue::List { elements: mapped })
    } else {
        Err("Invalid argument for map()".to_string())
    }
}

pub fn __filter(
    value: RuntimeValue,
    env: Rc<RefCell<Environment>>,
) -> Result<RuntimeValue, String> {
    if let RuntimeValue::List { elements: map_args } = value
        && let Some(RuntimeValue::Function { .. }) = map_args.first()
        && let Some(RuntimeValue::List {
            elements: original_items,
        }) = map_args.get(1)
    {
        let mut filtered = Vec::new();
        for item in original_items {
            let truth_value = Interpreter::interpret_function(
                map_args[0].clone(),
                item.clone(),
                Rc::clone(&env),
            )?;
            if truth_value.is_truthy() {
                filtered.push(item.clone());
            }
        }
        Ok(RuntimeValue::List { elements: filtered })
    } else {
        Err("Invalid argument for filter()".to_string())
    }
}

pub fn __foldr(value: RuntimeValue, env: Rc<RefCell<Environment>>) -> Result<RuntimeValue, String> {
    if let RuntimeValue::List { elements: map_args } = value
        && let Some(RuntimeValue::Function { .. }) = map_args.first()
        && let Some(initial) = map_args.get(1)
        && let Some(RuntimeValue::List {
            elements: original_items,
        }) = map_args.get(2)
    {
        let mut reduced = initial.clone();
        for value in original_items.iter().rev() {
            let with_left_arg = Interpreter::interpret_function(
                map_args[0].clone(),
                value.clone(),
                Rc::clone(&env),
            )?;
            reduced = Interpreter::interpret_function(with_left_arg, reduced, Rc::clone(&env))?;
        }
        Ok(reduced)
    } else {
        Err("Invalid argument for reduce()".to_string())
    }
}

pub fn __foldl(value: RuntimeValue, env: Rc<RefCell<Environment>>) -> Result<RuntimeValue, String> {
    if let RuntimeValue::List { elements: map_args } = value
        && let Some(RuntimeValue::Function { .. }) = map_args.first()
        && let Some(initial) = map_args.get(1)
        && let Some(RuntimeValue::List {
            elements: original_items,
        }) = map_args.get(2)
    {
        let mut reduced = initial.clone();
        for value in original_items.iter() {
            let with_right_arg =
                Interpreter::interpret_function(map_args[0].clone(), reduced, Rc::clone(&env))?;
            reduced =
                Interpreter::interpret_function(with_right_arg, value.clone(), Rc::clone(&env))?;
        }
        Ok(reduced)
    } else {
        Err("Invalid argument for reduce()".to_string())
    }
}

pub fn __len(value: RuntimeValue, _env: Rc<RefCell<Environment>>) -> Result<RuntimeValue, String> {
    match value {
        RuntimeValue::List { elements } => Ok(RuntimeValue::Number(elements.len() as f64)),
        _ => Err("len() expects a list".into()),
    }
}
