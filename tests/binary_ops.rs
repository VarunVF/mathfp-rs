use mathfp::interpreter::Interpreter;
use mathfp::runtime::RuntimeValue;
use mathfp::{execute_env_or_panic, execute_or_panic};

// assert helper
fn assert_bool(interpreter: &Interpreter, name: &str, cond: bool) {
    assert_eq!(
        interpreter.value_of(name),
        Some(RuntimeValue::Boolean(cond))
    )
}

#[test]
fn test_numeric_ops() {
    let input = "42 + 5 - 3 / 2 + 1 * 5";
    assert_eq!(execute_or_panic(input), RuntimeValue::Number(50.5));
}

#[test]
fn test_numeric_comparison() {
    let interpreter = Interpreter::new();
    let input = "
        nums_lt := 2 < 3;
        nums_le := 2 <= 2;
        nums_gt := 4 > 3;
        nums_ge := 4 >= 4;
        nums_ne := 12 != 71;
        nums_eq := 42 == 42;
    ";
    execute_env_or_panic(input, &interpreter);

    assert_bool(&interpreter, "nums_lt", true);
    assert_bool(&interpreter, "nums_le", true);
    assert_bool(&interpreter, "nums_gt", true);
    assert_bool(&interpreter, "nums_ge", true);
    assert_bool(&interpreter, "nums_ne", true);
    assert_bool(&interpreter, "nums_eq", true);
}

#[test]
fn test_string_op() {
    let input = "\"hello\" + \" world\"";
    assert_eq!(
        execute_or_panic(input),
        RuntimeValue::String("hello world".into())
    );
}

#[test]
fn test_different_type_compare() {
    let input = "5 == 5";
    assert_eq!(execute_or_panic(input), RuntimeValue::Boolean(true));

    let input = "sin >= \"sin\"";
    assert_eq!(execute_or_panic(input), RuntimeValue::Boolean(false));
}

#[test]
#[should_panic(expected = "Unsupported operands for '*'")]
fn test_invalid_type_op() {
    let input = "\"hello\" * 67";
    execute_or_panic(input);
}

#[test]
fn test_nil_ops() {
    let interpreter = Interpreter::new();
    let input = "
        ints_eq := 5 == 5;          // true
        nums_eq := 5 == 5.0;        // true (using f64 internally)
        nil_eq := nil == nil;       // true
        types_eq := 5 == nil;       // false
    ";
    execute_env_or_panic(input, &interpreter);

    assert_bool(&interpreter, "ints_eq", true);
    assert_bool(&interpreter, "nums_eq", true);
    assert_bool(&interpreter, "nil_eq", true);
    assert_bool(&interpreter, "types_eq", false);
}

#[test]
fn test_off_by_one() {
    let interpreter = Interpreter::new();
    let input = "
        test1 := 10 > 5;            // true
        test2 := 10 >= 10;          // true
        test3 := 5 < 10;            // true
        test4 := 5 <= 5;            // true
        test5 := 5 != 10;           // true
    ";
    execute_env_or_panic(input, &interpreter);

    assert_bool(&interpreter, "test1", true);
    assert_bool(&interpreter, "test2", true);
    assert_bool(&interpreter, "test3", true);
    assert_bool(&interpreter, "test4", true);
    assert_bool(&interpreter, "test5", true);
}
