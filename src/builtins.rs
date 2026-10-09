use std::{
    cell::RefCell,
    fs,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    interpreter::Interpreter,
    runtime::{Environment, RuntimeValue},
};

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

pub fn __read_file(
    value: RuntimeValue,
    _env: Rc<RefCell<Environment>>,
) -> Result<RuntimeValue, String> {
    if let RuntimeValue::String(filename) = value {
        let contents = fs::read_to_string(&filename)
            .map_err(|e| format!("Failed to open file '{filename}': {e}"))?;
        Ok(RuntimeValue::String(contents))
    } else {
        Err(format!(
            "read_file() expects a filename string, not {}",
            value.type_str()
        ))
    }
}

pub fn __write_file(
    value: RuntimeValue,
    _env: Rc<RefCell<Environment>>,
) -> Result<RuntimeValue, String> {
    if let RuntimeValue::List { elements: args } = value
        && let Some(RuntimeValue::String(filename)) = args.first()
        && let Some(RuntimeValue::String(text)) = args.get(1)
    {
        fs::write(filename, text).map_err(|e| e.to_string())?;
        Ok(RuntimeValue::Nil)
    } else {
        Err("Invalid argument for write_file()".to_string())
    }
}

pub fn __sin(value: RuntimeValue, _env: Rc<RefCell<Environment>>) -> Result<RuntimeValue, String> {
    match value {
        RuntimeValue::Number(n) => Ok(RuntimeValue::Number(n.sin())),
        _ => Err(format!("sin() expects a number, not {}", value.type_str())),
    }
}

pub fn __cos(value: RuntimeValue, _env: Rc<RefCell<Environment>>) -> Result<RuntimeValue, String> {
    match value {
        RuntimeValue::Number(n) => Ok(RuntimeValue::Number(n.cos())),
        _ => Err(format!("cos() expects a number, not {}", value.type_str())),
    }
}

pub fn __sqrt(value: RuntimeValue, _env: Rc<RefCell<Environment>>) -> Result<RuntimeValue, String> {
    match value {
        RuntimeValue::Number(n) => Ok(RuntimeValue::Number(n.sqrt())),
        _ => Err(format!("sqrt() expects a number, not {}", value.type_str())),
    }
}

pub fn __floor(
    value: RuntimeValue,
    _env: Rc<RefCell<Environment>>,
) -> Result<RuntimeValue, String> {
    match value {
        RuntimeValue::Number(n) => Ok(RuntimeValue::Number(n.floor())),
        _ => Err(format!(
            "floor() expects a number, not {}",
            value.type_str()
        )),
    }
}

pub fn __ceil(value: RuntimeValue, _env: Rc<RefCell<Environment>>) -> Result<RuntimeValue, String> {
    match value {
        RuntimeValue::Number(n) => Ok(RuntimeValue::Number(n.ceil())),
        _ => Err(format!("ceil() expects a number, not {}", value.type_str())),
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

pub fn __type(value: RuntimeValue, _env: Rc<RefCell<Environment>>) -> Result<RuntimeValue, String> {
    Ok(RuntimeValue::String(value.type_str().to_string()))
}

pub fn __bool(value: RuntimeValue, _env: Rc<RefCell<Environment>>) -> Result<RuntimeValue, String> {
    // Convert to a boolean runtime value
    Ok(RuntimeValue::Boolean(value.is_truthy()))
}

pub fn __str(value: RuntimeValue, _env: Rc<RefCell<Environment>>) -> Result<RuntimeValue, String> {
    // Convert to a string runtime value
    Ok(RuntimeValue::String(value.to_string()))
}

pub fn __map(value: RuntimeValue, env: Rc<RefCell<Environment>>) -> Result<RuntimeValue, String> {
    if let RuntimeValue::List { elements: args } = value
        && let Some(RuntimeValue::Function { .. }) = args.first()
        && let Some(RuntimeValue::List {
            elements: original_items,
        }) = args.get(1)
    {
        let mut mapped = Vec::with_capacity(original_items.len());
        for item in original_items {
            let new_value =
                Interpreter::interpret_function(args[0].clone(), item.clone(), Rc::clone(&env));
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
    if let RuntimeValue::List { elements: args } = value
        && let Some(RuntimeValue::Function { .. }) = args.first()
        && let Some(RuntimeValue::List {
            elements: original_items,
        }) = args.get(1)
    {
        let mut filtered = Vec::with_capacity(original_items.len());
        for item in original_items {
            let truth_value =
                Interpreter::interpret_function(args[0].clone(), item.clone(), Rc::clone(&env))?;
            if truth_value.is_truthy() {
                filtered.push(item.clone());
            }
        }
        Ok(RuntimeValue::List { elements: filtered })
    } else {
        Err("Invalid argument for filter()".to_string())
    }
}

pub fn __foldl(value: RuntimeValue, env: Rc<RefCell<Environment>>) -> Result<RuntimeValue, String> {
    if let RuntimeValue::List { elements: args } = value
        && let Some(RuntimeValue::Function { .. }) = args.first()
        && let Some(initial) = args.get(1)
        && let Some(RuntimeValue::List {
            elements: original_items,
        }) = args.get(2)
    {
        let mut reduced = initial.clone();
        for value in original_items.iter() {
            let with_right_arg =
                Interpreter::interpret_function(args[0].clone(), reduced, Rc::clone(&env))?;
            reduced =
                Interpreter::interpret_function(with_right_arg, value.clone(), Rc::clone(&env))?;
        }
        Ok(reduced)
    } else {
        Err("Invalid argument for reduce() / foldl()".to_string())
    }
}

pub fn __foldr(value: RuntimeValue, env: Rc<RefCell<Environment>>) -> Result<RuntimeValue, String> {
    if let RuntimeValue::List { elements: args } = value
        && let Some(RuntimeValue::Function { .. }) = args.first()
        && let Some(initial) = args.get(1)
        && let Some(RuntimeValue::List {
            elements: original_items,
        }) = args.get(2)
    {
        let mut reduced = initial.clone();
        for value in original_items.iter().rev() {
            let with_left_arg =
                Interpreter::interpret_function(args[0].clone(), value.clone(), Rc::clone(&env))?;
            reduced = Interpreter::interpret_function(with_left_arg, reduced, Rc::clone(&env))?;
        }
        Ok(reduced)
    } else {
        Err("Invalid argument for foldr()".to_string())
    }
}

pub fn __len(value: RuntimeValue, _env: Rc<RefCell<Environment>>) -> Result<RuntimeValue, String> {
    let length = match value {
        RuntimeValue::List { elements } => elements.len(),
        RuntimeValue::String(elements) => elements.len(),
        _ => Err(format!(
            "len() expects a list or str, not {}",
            value.type_str()
        ))?,
    };
    Ok(RuntimeValue::Number(length as f64))
}

fn is_whole(x: f64) -> bool {
    x.fract() == 0.0
}

pub fn __get(value: RuntimeValue, _env: Rc<RefCell<Environment>>) -> Result<RuntimeValue, String> {
    if let RuntimeValue::List { elements: args } = value
        && let Some(RuntimeValue::List { .. } | RuntimeValue::String(_)) = args.first()
        && let Some(&RuntimeValue::Number(i)) = args.get(1)
    {
        let index = if is_whole(i) {
            i as usize
        } else {
            Err("Indices must be integers, not float".to_string())?
        };
        let elements = &args[0];
        elements.get(index).ok_or(format!(
            "Index {} is out of bounds for {} of length {}",
            index,
            elements.type_str(),
            elements.len()?
        ))
    } else {
        Err("Invalid argument for get()".to_string())
    }
}

fn verify_and_get_slice(start: f64, stop: f64) -> Result<(usize, usize), String> {
    if !is_whole(start) || !is_whole(stop) {
        Err(format!(
            "Range bounds must be integers, not [{start}, {stop})"
        ))
    } else if start > stop {
        Err(format!("Range [{start}, {stop}) is empty"))
    } else {
        Ok((start as usize, stop as usize))
    }
}

pub fn __slice(
    value: RuntimeValue,
    _env: Rc<RefCell<Environment>>,
) -> Result<RuntimeValue, String> {
    if let RuntimeValue::List { elements: args } = value
        && let Some(&RuntimeValue::Number(start)) = args.get(1)
        && let Some(&RuntimeValue::Number(stop)) = args.get(2)
    {
        let (start, stop) = verify_and_get_slice(start, stop)?;
        if let Some(RuntimeValue::String(str)) = args.first() {
            Ok(RuntimeValue::String(
                str.char_indices()
                    .filter(|(idx, _)| *idx >= start && *idx < stop)
                    .map(|(_, ch)| ch)
                    .collect(),
            ))
        } else if let Some(RuntimeValue::List { elements: items }) = args.first() {
            Ok(RuntimeValue::List {
                elements: items.get(start..stop).unwrap_or(&[]).to_vec(),
            })
        } else {
            Err("Only strings and lists can be passed to slice()".to_string())
        }
    } else {
        Err("Invalid argument for slice()".to_string())
    }
}

pub fn __split(
    value: RuntimeValue,
    _env: Rc<RefCell<Environment>>,
) -> Result<RuntimeValue, String> {
    if let RuntimeValue::List { elements: args } = value
        && let Some(RuntimeValue::String(str)) = args.first()
        && let Some(RuntimeValue::String(sep)) = args.get(1)
    {
        Ok(RuntimeValue::List {
            elements: str
                .split(sep)
                .map(|e| RuntimeValue::String(e.to_string()))
                .collect(),
        })
    } else {
        Err("Invalid argument for split()".to_string())
    }
}

pub fn __join(value: RuntimeValue, _env: Rc<RefCell<Environment>>) -> Result<RuntimeValue, String> {
    if let RuntimeValue::List { elements: args } = value
        && let Some(RuntimeValue::List { elements }) = args.first()
        && let Some(RuntimeValue::String(sep)) = args.get(1)
    {
        let mut joined = String::new();
        for i in 0..elements.len() {
            if let RuntimeValue::String(str) = &elements[i] {
                joined += str;
                if i != elements.len() - 1 {
                    joined += sep;
                }
            } else {
                return Err(format!(
                    "join() list items must be strings, not {}",
                    elements[i].type_str()
                ));
            }
        }
        Ok(RuntimeValue::String(joined))
    } else {
        Err("Invalid argument for join()".to_string())
    }
}

fn verify_and_get_range(start: f64, stop: f64) -> Result<(i64, i64), String> {
    if !is_whole(start) || !is_whole(stop) {
        Err(format!(
            "Range bounds must be integers, not [{start}, {stop})"
        ))
    } else if start > stop {
        Err(format!("Range [{start}, {stop}) is empty"))
    } else {
        Ok((start as i64, stop as i64))
    }
}

pub fn __range(
    value: RuntimeValue,
    _env: Rc<RefCell<Environment>>,
) -> Result<RuntimeValue, String> {
    if let RuntimeValue::List { elements: args } = value
        && let Some(&RuntimeValue::Number(start)) = args.first()
        && let Some(&RuntimeValue::Number(stop)) = args.get(1)
    {
        let (start, stop) = verify_and_get_range(start, stop)?;
        Ok(RuntimeValue::List {
            elements: (start..stop)
                .map(|x| RuntimeValue::Number(x as f64))
                .collect(),
        })
    } else {
        Err("Invalid argument for range()".to_string())
    }
}
