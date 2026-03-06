use std::cell::RefCell;
use std::rc::Rc;

use mathfp::runtime::{Environment, RuntimeValue};
use mathfp::{execute_env, execute_env_or_panic, execute_or_panic};

#[test]
fn test_numeric_ops() {
    let input = "42 + 5 - 3 / 2 + 1 * 5";
    assert_eq!(execute_or_panic(input), RuntimeValue::Number(50.5));
}

#[test]
fn test_numeric_comparison() {
    let env = Rc::new(RefCell::new(Environment::new()));
    let input = "
        nums_lt := 2 < 3;
        nums_le := 2 <= 2;
        nums_gt := 4 > 3;
        nums_ge := 4 >= 4;
        nums_ne := 12 != 71;
        nums_eq := 42 == 42;
        final := true == nums_lt == nums_le == nums_gt == nums_ge == nums_ne == nums_eq
        final
    ";
    assert_eq!(
        execute_env_or_panic(input, Rc::clone(&env)),
        RuntimeValue::Boolean(true)
    );
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
    let env = Rc::new(RefCell::new(Environment::new()));
    let input = "
        test1 := 5 == 5;        // Should be true
        test2 := 5 == 5.0;      // Should be true (using f64 internally)
        test3 := nil == nil;    // Should be true
        test4 := 5 == nil;      // Should be false
    ";

    execute_env(input, Rc::clone(&env)).unwrap();

    assert_eq!(
        env.borrow().resolve("test1"),
        Some(RuntimeValue::Boolean(true))
    );
    assert_eq!(
        env.borrow().resolve("test2"),
        Some(RuntimeValue::Boolean(true))
    );
    assert_eq!(
        env.borrow().resolve("test3"),
        Some(RuntimeValue::Boolean(true))
    );
    assert_eq!(
        env.borrow().resolve("test4"),
        Some(RuntimeValue::Boolean(false))
    );
}

#[test]
fn test_off_by_one() {
    let env = Rc::new(RefCell::new(Environment::new()));
    let input = "
        test1 := 10 > 5;        // true
        test2 := 10 >= 10;      // true
        test3 := 5 < 10;        // true
        test4 := 5 <= 5;        // true
        test5 := 5 != 10;       // true
    ";

    execute_env(input, Rc::clone(&env)).unwrap();

    let true_val = Some(RuntimeValue::Boolean(true));
    assert_eq!(env.borrow().resolve("test1"), true_val);
    assert_eq!(env.borrow().resolve("test2"), true_val);
    assert_eq!(env.borrow().resolve("test3"), true_val);
    assert_eq!(env.borrow().resolve("test4"), true_val);
    assert_eq!(env.borrow().resolve("test5"), true_val);
}
