use mathfp::interpreter::Interpreter;
use mathfp::runtime::RuntimeValue;
use mathfp::{execute_env, execute_env_or_panic};

#[test]
fn test_nested_block() {
    let interpreter = Interpreter::new();
    let input = "x := 0; { x := 6; { x = 7; }; x }";
    let value = execute_env_or_panic(input, &interpreter);
    assert_eq!(value, RuntimeValue::Number(7.0));

    let value = execute_env_or_panic("x", &interpreter);
    assert_eq!(value, RuntimeValue::Number(0.0));
}

#[test]
fn test_block_isolation() {
    let interpreter = Interpreter::new();

    let input1 = "x := 10; y := { x := 20; x }; result := x;";
    execute_env_or_panic(input1, &interpreter);
    assert_eq!(
        execute_env("y", &interpreter),
        Ok(RuntimeValue::Number(20.0))
    );
    assert_eq!(
        execute_env("result", &interpreter),
        Ok(RuntimeValue::Number(10.0))
    );
}
