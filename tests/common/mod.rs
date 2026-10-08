#![allow(dead_code)]

use mathfp::{execute_or_panic, runtime::RuntimeValue};

pub fn assert_value(input: &str, expected: RuntimeValue) {
    assert_eq!(execute_or_panic(input, &[]), expected)
}

pub fn assert_bool(input: &str, expected: bool) {
    assert_value(input, RuntimeValue::Boolean(expected))
}

pub fn assert_str(input: &str, expected: &str) {
    assert_value(input, RuntimeValue::String(expected.to_string()))
}

pub fn assert_list(input: &str, expected: &[RuntimeValue]) {
    assert_value(
        input,
        RuntimeValue::List {
            elements: expected.to_vec(),
        },
    )
}

pub fn assert_number(input: &str, expected: f64) {
    assert_value(input, RuntimeValue::Number(expected))
}

pub fn assert_nil(input: &str) {
    assert_value(input, RuntimeValue::Nil)
}
