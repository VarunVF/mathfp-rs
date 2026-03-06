use std::cell::RefCell;
use std::rc::Rc;

use mathfp::execute_env;
use mathfp::runtime::{Environment, RuntimeValue};

#[test]
fn test_unary_op_number() {
    let env = Rc::new(RefCell::new(Environment::new()));
    let input = "
        test1 := !(!1);     // true (if 1 is truthy)
        test2 := -5 < 0;    // true (Tests Unary -)
    ";

    execute_env(input, Rc::clone(&env)).unwrap();

    let true_val = Some(RuntimeValue::Boolean(true));
    assert_eq!(env.borrow().resolve("test1"), true_val);
    assert_eq!(env.borrow().resolve("test2"), true_val);
}

#[test]
fn test_unary_op_boolean() {
    let env = Rc::new(RefCell::new(Environment::new()));
    let input = "test1 := !false;";

    let result = execute_env(input, Rc::clone(&env)).unwrap();
    assert_eq!(result, RuntimeValue::Boolean(true));
}
