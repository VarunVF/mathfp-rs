use mathfp::interpreter::Interpreter;
use mathfp::runtime::RuntimeValue;
use mathfp::{execute_env, execute_env_or_panic, execute_or_panic};

#[test]
fn test_unary_op_number() {
    let interpreter = Interpreter::new();
    let input = "
        test1 := !(!1);     // true (if 1 is truthy)
        test2 := -5 < 0;    // true (Tests Unary -)
    ";

    execute_env(input, &interpreter).unwrap();

    let true_val = RuntimeValue::Boolean(true);
    assert_eq!(execute_env_or_panic("test1", &interpreter), true_val);
    assert_eq!(execute_env_or_panic("test2", &interpreter), true_val);
}

#[test]
fn test_unary_op_boolean() {
    let input = "test1 := !false;";

    let result = execute_or_panic(input);
    assert_eq!(result, RuntimeValue::Boolean(true));
}
