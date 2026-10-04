#![allow(dead_code)]

use mathfp::{execute_or_panic, runtime::RuntimeValue};

pub fn assert_value(input: &str, expected: RuntimeValue) {
    assert_eq!(execute_or_panic(input, &[]), expected);
}

pub fn assert_bool(input: &str, cond: bool) {
    assert_eq!(execute_or_panic(input, &[]), RuntimeValue::Boolean(cond))
}
