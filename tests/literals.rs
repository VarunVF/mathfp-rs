mod common;

use crate::common::assert_value;
use mathfp::{execute, execute_or_panic, runtime::RuntimeValue};

#[test]
fn test_number() {
    assert_value("10.", RuntimeValue::Number(10.0));
    assert_value(".05", RuntimeValue::Number(0.05));
    assert_value("7.2", RuntimeValue::Number(7.2));
}

#[test]
#[should_panic(expected = "Failed to parse '..' as a number")]
fn test_invalid_number() {
    execute_or_panic("..", &[]);
}

#[test]
fn test_string() {
    assert_value(
        // value of literal: "\thello, world!\n"
        "\"\\thello, world!\\n\"",
        RuntimeValue::String("\thello, world!\n".to_string()),
    );
    assert_value(
        // value of literal: "\"quoted\""
        "\"\\\"quoted\\\"\"",
        RuntimeValue::String("\"quoted\"".to_string()),
    );
}

#[test]
fn test_boolean() {
    assert_value("true", RuntimeValue::Boolean(true));
    assert_value("false", RuntimeValue::Boolean(false));
}

#[test]
fn test_lambda() {
    let input = "(x |-> x + 1)(10)";
    assert_eq!(execute(input, &[]), Ok(RuntimeValue::Number(11.0)));
}

#[test]
fn test_function() {
    assert_value("x := _ |-> 42; x()", RuntimeValue::Number(42.0));
}

#[test]
fn test_list() {
    assert_value("[]", RuntimeValue::List { elements: vec![] });
    assert_value(
        "[1, 2, []]",
        RuntimeValue::List {
            elements: vec![
                RuntimeValue::Number(1.0),
                RuntimeValue::Number(2.0),
                RuntimeValue::List { elements: vec![] },
            ],
        },
    );
}

#[test]
fn test_nil() {
    assert_value("nil", RuntimeValue::Nil);
}
