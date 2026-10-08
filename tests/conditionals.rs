mod common;

use common::{assert_nil, assert_number, assert_str};
use mathfp::interpreter::Interpreter;
use mathfp::runtime::RuntimeValue;
use mathfp::{execute_env_or_panic, execute_or_panic};

#[test]
#[should_panic(expected = "Expected at least one match arm")]
fn test_empty_match() {
    let input = "
        divide := a |-> b |-> match {};
        result := divide(2)(1);
    ";

    execute_or_panic(input, &[]);
}

#[test]
fn test_match_exhaustive() {
    let interpreter = Interpreter::default();
    let input = "
        divide := a |-> b |-> match {
            b == 0 => nil,
            b != 0 => a / b,
        };
        valid   := divide(2)(1);
        invalid := divide(2)(0);
    ";

    execute_env_or_panic(input, &interpreter);
    assert_eq!(
        execute_env_or_panic("valid", &interpreter),
        RuntimeValue::Number(2.0)
    );
    assert_eq!(
        execute_env_or_panic("invalid", &interpreter),
        RuntimeValue::Nil
    );
}

#[test]
fn test_match_non_exhaustive() {
    let input = "
        divide := a |-> b |-> match {
            b != 0 => a / b,
        };
        result := divide(2)(0);
        result
    ";

    assert_nil(input);
}

#[test]
fn test_match_early_return() {
    let input = "
        x := 10;
        result := match {
            x > 5  => 1,
            x > 0  => 2,  // Also true, but should be ignored
        }
    ";

    assert_number(input, 1.0);
}

#[test]
fn test_nested_match() {
    let input = "
        x := 1;
        y := 2;
        result := match {
            x == 1 => match {
                y == 2 => \"both\",
                y != 2 => \"just x\"
            },
            x != 1 => \"none\"
        };
    ";

    assert_str(input, "both");
}

#[test]
fn test_if_expr() {
    // dangling else
    assert_number("if true then if false then 1 else 2", 2.0);
    assert_nil("if false then if false then 1 else 2");

    // expression nesting
    assert_number("x := 10; 5 + (if x then 10 else 0)", 15.0);
}
