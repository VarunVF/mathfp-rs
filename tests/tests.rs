use mathfp::runtime::{Environment, RuntimeValue};
use mathfp::{execute, execute_env_or_panic, execute_or_panic};

use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_complex_conditional_logic() {
    // dangling else
    let input = "if true then if false then 1 else 2";
    assert_eq!(execute(input), Ok(RuntimeValue::Number(2.0)));

    // dangling else
    let input = "if false then if false then 1 else 2";
    assert_eq!(execute(input), Ok(RuntimeValue::Nil));

    // expression nesting
    let input2 = "x := 10; 5 + (if x then 10 else 0)";
    assert_eq!(execute(input2), Ok(RuntimeValue::Number(15.0)));
}

#[test]
fn test_lambda() {
    let input = "f := x |-> x + 1; f(10)";
    assert_eq!(execute(input), Ok(RuntimeValue::Number(11.0)));
}

#[test]
fn test_outer_scope_binding_change() {
    let input = "
        x := 10;
        f := y |-> x + y;
        x = 20;
        f(5)
    ";
    // If lexical scoping works, f(5) uses x=20 (the latest global).
    // In most functional languages, it should be 25.0.
    assert_eq!(execute(input), Ok(RuntimeValue::Number(25.0)));
}

#[test]
fn test_closures_and_higher_order() {
    let input = "
        make_adder := x |-> (y |-> x + y);
        add5 := make_adder(5);
        add10 := make_adder(10);
        add5(2) + add10(2)
    ";
    // (5 + 2) + (10 + 2) = 7 + 12 = 19
    assert_eq!(execute(input), Ok(RuntimeValue::Number(19.0)));
}

#[test]
fn test_nested_shadowing() {
    let env = Rc::new(RefCell::new(Environment::new()));
    let input = "
        val := 100;
        f := x |-> val := x + x;
        f(10)
    ";

    assert_eq!(
        execute_env_or_panic(input, Rc::clone(&env)),
        RuntimeValue::Number(20.0)
    );
    // Check that global val is still 100
    assert_eq!(
        env.borrow().resolve("val"),
        Some(RuntimeValue::Number(100.0))
    );
}

#[test]
fn test_function_composition() {
    let input = "
        square := x |-> x * x;
        double := x |-> x + x;
        square(double(5))
    ";
    assert_eq!(execute(input), Ok(RuntimeValue::Number(100.0)));
}

#[test]
fn test_if_returning_function() {
    let input = "
        f := if true then (x |-> x + 1) else (x |-> x - 1);
        f(10)
    ";
    assert_eq!(execute(input), Ok(RuntimeValue::Number(11.0)));
}

#[test]
fn test_recursion() {
    let input = "
        fact := n |-> if n then n*fact(n-1) else 1;
        fact(5)
    ";
    assert_eq!(execute(input), Ok(RuntimeValue::Number(120.0)));
}

#[test]
fn test_mutual_recursion() {
    let input = "
        is_even := n |-> if n then is_odd(n - 1) else true;
        is_odd := n |-> if n then is_even(n - 1) else false;
        is_even(4)
    ";

    assert_eq!(execute(input), Ok(RuntimeValue::Boolean(true)));
}

#[test]
fn test_closure_equality() {
    // Closures should not be equal
    let env = Rc::new(RefCell::new(Environment::new()));

    execute_env_or_panic("make_adder := x |-> (y |-> x + y)", Rc::clone(&env));

    let add5 = execute_env_or_panic("make_adder(5)", Rc::clone(&env));
    let add10 = execute_env_or_panic("make_adder(10)", Rc::clone(&env));

    assert_ne!(
        add5, add10,
        "Functions with different captured closures should not be equal"
    );
}

#[test]
fn test_lexical_scope_isolation() {
    let input = "
        x := 10;
        f := n |-> x + n;
        
        caller := dummy |-> {
x := 100;
            f(5)
        };
        
        caller(nil)
    ";

    // If lexical, it must be 15.0 (10 + 5)
    assert_eq!(execute(input), Ok(RuntimeValue::Number(15.0)));
}

#[test]
fn test_variable_scope() {
    let input = "
        x := 5;
        (dummy |-> x = 7)(nil);
        x
    ";

    assert_eq!(execute(input), Ok(RuntimeValue::Number(7.0)));
}

#[test]
fn test_closure_mutation_counter() {
    let input = "
        make_counter := start |-> {
            val := start;
            n |-> { 
                val = val + n; 
                val 
            }
        };
        c := make_counter(0);
        c(1);
        c(1);
        c(5)
    ";

    // The final call c(5) should return 1 + 1 + 5 = 7
    assert_eq!(execute(input), Ok(RuntimeValue::Number(7.0)));
}

#[test]
fn test_comments() {
    let input = "
        x := 10; // This is a comment
        // This is a whole line comment
        y := 5;
        x + y // Returns 15
    ";

    assert_eq!(execute(input), Ok(RuntimeValue::Number(15.0)));
}

#[test]
fn test_comments_inside_functions() {
    let input = "
        // Squares a number.
        f := x |-> {
            // Calculate square
            res := x * x;
            res // return it
        };
        f(4)
    ";

    assert_eq!(execute(input), Ok(RuntimeValue::Number(16.0)));
}

#[test]
#[should_panic(expected = "Function body cannot be empty, use {} instead")]
fn test_function_body_empty_error() {
    let input = "f := _ |-> ;";

    execute_or_panic(input);
}

#[test]
fn test_function_body_empty_ok() {
    let input = "f := _ |-> {}";

    assert!(matches!(execute(input), Ok(RuntimeValue::Function { .. })));
}

#[test]
#[should_panic(expected = "Name 'no_such_function' is not defined")]
fn test_undefined_name_in_function() {
    let input = "
        main := _ |-> {
            value := no_such_function(nil);
            println(value);
        }
        main(nil)
    ";

    // Evaluation should stop as soon as `no_such_function` was found to be unresolved.
    // It should never go on to the next statement.
    execute_or_panic(input);
}

#[test]
#[should_panic(expected = "Expected an expression")]
fn test_incomplete_function_def() {
    let input = "f := x |-> {";
    execute_or_panic(input);
}

#[test]
#[should_panic]
fn test_incomplete_grouping() {
    let input = "f := x + (1";
    execute_or_panic(input);
}
