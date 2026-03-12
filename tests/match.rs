use mathfp::interpreter::Interpreter;
use mathfp::runtime::RuntimeValue;
use mathfp::{execute_env, execute_env_or_panic};

#[test]
#[should_panic(expected = "Expected at least one match arm")]
fn test_empty_match() {
    let interpreter = Interpreter::new();
    let input = "
        divide := a |-> b |-> match {};
        result := divide(2)(1);
    ";
    execute_env_or_panic(input, &interpreter);
}

#[test]
fn test_match_exhaustive() {
    let interpreter = Interpreter::new();
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
        execute_env("valid", &interpreter),
        Ok(RuntimeValue::Number(2.0))
    );
    assert_eq!(execute_env("invalid", &interpreter), Ok(RuntimeValue::Nil));
}

#[test]
fn test_match_non_exhaustive() {
    let interpreter = Interpreter::new();
    let input = "
        divide := a |-> b |-> match {
            b != 0 => a / b,
        };
        result := divide(2)(0);
    ";
    execute_env_or_panic(input, &interpreter);

    assert_eq!(execute_env("result", &interpreter), Ok(RuntimeValue::Nil));
}

#[test]
fn test_match_early_return() {
    let interpreter = Interpreter::new();
    let input = "
        x := 10;
        result := match {
            x > 5  => 1,
            x > 0  => 2,  // Also true, but should be ignored
        }
    ";

    assert_eq!(
        execute_env(input, &interpreter),
        Ok(RuntimeValue::Number(1.0))
    );
}

#[test]
fn test_nested_match() {
    let interpreter = Interpreter::new();
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

    execute_env_or_panic(input, &interpreter);
    assert_eq!(
        execute_env("result", &interpreter),
        Ok(RuntimeValue::String("both".to_string()))
    );
}
