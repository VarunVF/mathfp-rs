mod common;

use common::{assert_bool, assert_list, assert_nil, assert_number, assert_str};
use mathfp::{execute, execute_or_panic, runtime::RuntimeValue};

#[test]
fn test_number() {
    assert_number("10.", 10.0);
    assert_number(".05", 0.05);
    assert_number("7.2", 7.2);
}

#[test]
#[should_panic(expected = "Numeric literals must have at most one decimal point")]
fn test_invalid_number() {
    execute_or_panic("..", &[]);
}

#[test]
fn test_string() {
    // value of literal: "\thello, world!\n"
    assert_str("\"\\thello, world!\\n\"", "\thello, world!\n");
    // value of literal: "\"quoted\""
    assert_str("\"\\\"quoted\\\"\"", "\"quoted\"");
}

#[test]
fn test_boolean() {
    assert_bool("true", true);
    assert_bool("false", false);
}

#[test]
fn test_lambda() {
    let input = "(x |-> x + 1)(10)";
    assert_eq!(execute(input, &[]), Ok(RuntimeValue::Number(11.0)));
}

#[test]
fn test_function() {
    assert_number("x := _ |-> 42; x()", 42.0);
}

#[test]
fn test_list() {
    assert_list("[]", &[]);
    assert_list(
        "[1, 2, []]",
        &[
            RuntimeValue::Number(1.0),
            RuntimeValue::Number(2.0),
            RuntimeValue::List { elements: vec![] },
        ],
    );
}

#[test]
fn test_nil() {
    assert_nil("nil");
}
