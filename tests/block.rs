mod common;

use common::assert_nil;
use mathfp::interpreter::Interpreter;
use mathfp::runtime::RuntimeValue;
use mathfp::{execute_env, execute_env_or_panic};

#[test]
fn test_nested_block() {
    let interpreter = Interpreter::default();

    assert_eq!(
        execute_env_or_panic("x := 0; { x := 6; { x = 7; }; x }", &interpreter),
        RuntimeValue::Number(7.0)
    );
    assert_eq!(
        execute_env_or_panic("x", &interpreter),
        RuntimeValue::Number(0.0)
    );
}

#[test]
fn test_block_isolation() {
    let interpreter = Interpreter::default();

    execute_env_or_panic("x := 10; y := { x := 20; x }; result := x;", &interpreter);
    assert_eq!(
        execute_env("y", &interpreter),
        Ok(RuntimeValue::Number(20.0))
    );
    assert_eq!(
        execute_env("result", &interpreter),
        Ok(RuntimeValue::Number(10.0))
    );
}

#[test]
fn test_empty_block() {
    assert_nil("f := _ |-> {}; f()");
}
