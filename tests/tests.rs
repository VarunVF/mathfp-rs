use mathfp::eval;
use mathfp::parser::Parser;
use mathfp::runtime::{Environment, RuntimeValue};
use mathfp::token::Scanner;

use std::cell::RefCell;
use std::rc::Rc;

fn run(input: &str, env: Rc<RefCell<Environment>>) -> RuntimeValue {
    let tokens = Scanner::new(input)
        .scan()
        .expect("Scanner should scan correctly");
    let expr = Parser::new(tokens)
        .parse()
        .expect("Parser should parse correctly");
    eval::evaluate(expr, env).expect("Evaluation should run correctly")
}

#[test]
fn test_complex_conditional_logic() {
    let env = Rc::new(RefCell::new(Environment::new()));

    // dangling else
    let input = "if true then if false then 1 else 2";
    assert_eq!(run(input, Rc::clone(&env)), RuntimeValue::Number(2.0));

    // dangling else
    let input = "if false then if false then 1 else 2";
    assert_eq!(run(input, Rc::clone(&env)), RuntimeValue::Nil);

    // expression nesting
    let input2 = "x := 10; 5 + (if x then 10 else 0)";
    assert_eq!(run(input2, env), RuntimeValue::Number(15.0));
}

#[test]
fn test_lambda() {
    let env = Rc::new(RefCell::new(Environment::new()));
    let input = "f := x |-> x + 1; f(10)";
    assert_eq!(run(input, env), RuntimeValue::Number(11.0));
}

#[test]
fn test_outer_scope_binding_change() {
    let env = Rc::new(RefCell::new(Environment::new()));
    let input = "
        x := 10;
        f := y |-> x + y;
        x = 20;
        f(5)
    ";
    // If lexical scoping works, f(5) uses x=20 (the latest global).
    // In most functional languages, it should be 25.0.
    assert_eq!(run(input, env), RuntimeValue::Number(25.0));
}

#[test]
fn test_closures_and_higher_order() {
    let env = Rc::new(RefCell::new(Environment::new()));
    let input = "
        make_adder := x |-> (y |-> x + y);
        add5 := make_adder(5);
        add10 := make_adder(10);
        add5(2) + add10(2)
    ";
    // (5 + 2) + (10 + 2) = 7 + 12 = 19
    assert_eq!(run(input, env), RuntimeValue::Number(19.0));
}

#[test]
fn test_nested_shadowing() {
    let env = Rc::new(RefCell::new(Environment::new()));
    let input = "
        val := 100;
        f := x |-> val := x + x;
        f(10)
    ";

    assert_eq!(run(input, Rc::clone(&env)), RuntimeValue::Number(20.0));
    // Check that global val is still 100
    assert_eq!(
        env.borrow().resolve("val"),
        Some(RuntimeValue::Number(100.0))
    );
}

#[test]
fn test_function_composition() {
    let env = Rc::new(RefCell::new(Environment::new()));
    let input = "
        square := x |-> x * x;
        double := x |-> x + x;
        square(double(5))
    ";
    assert_eq!(run(input, env), RuntimeValue::Number(100.0));
}

#[test]
fn test_if_returning_function() {
    let env = Rc::new(RefCell::new(Environment::new()));
    let input = "
        f := if true then (x |-> x + 1) else (x |-> x - 1);
        f(10)
    ";
    assert_eq!(run(input, env), RuntimeValue::Number(11.0));
}

#[test]
fn test_recursion() {
    let env = Rc::new(RefCell::new(Environment::new()));
    let input = "
        fact := n |-> if n then n*fact(n-1) else 1;
        fact(5)
    ";
    assert_eq!(run(input, env), RuntimeValue::Number(120.0));
}

#[test]
fn test_mutual_recursion() {
    let env = Rc::new(RefCell::new(Environment::new()));

    let input = "
        is_even := n |-> if n then is_odd(n - 1) else true;
        is_odd := n |-> if n then is_even(n - 1) else false;
        is_even(4)
    ";

    assert_eq!(run(input, Rc::clone(&env)), RuntimeValue::Boolean(true));
}

#[test]
fn test_user_defined_function_equality() {
    let env = Rc::new(RefCell::new(Environment::new()));

    // Identical functions should be equal
    let f1 = run("x |-> x + 1", Rc::clone(&env));
    let f2 = run("x |-> x + 1", Rc::clone(&env));
    assert_eq!(f1, f2, "Identical lambdas should be equal");

    // Different parameter names should not be equal
    let f3 = run("y |-> y + 1", Rc::clone(&env));
    assert_ne!(f1, f3, "Different parameter names should not be equal");

    // Different bodies should not be equal
    let f4 = run("x |-> x + 2", Rc::clone(&env));
    assert_ne!(f1, f4, "Different bodies should not be equal");
}

#[test]
fn test_closure_equality() {
    let env = Rc::new(RefCell::new(Environment::new()));

    // Functions with different captured values in closures should NOT be equal
    run("make_adder := x |-> (y |-> x + y)", Rc::clone(&env));

    let add5 = run("make_adder(5)", Rc::clone(&env));
    let add10 = run("make_adder(10)", Rc::clone(&env));

    assert_ne!(
        add5, add10,
        "Functions with different captured closures should not be equal"
    );
}

#[test]
fn test_lexical_scope_isolation() {
    let env = Rc::new(RefCell::new(Environment::new()));

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
    assert_eq!(run(input, Rc::clone(&env)), RuntimeValue::Number(15.0));
}

#[test]
fn test_variable_scope() {
    let env = Rc::new(RefCell::new(Environment::new()));

    let input = "
        x := 5;
        (dummy |-> x = 7)(nil);
        x
    ";

    assert_eq!(run(input, Rc::clone(&env)), RuntimeValue::Number(7.0));
}

#[test]
fn test_closure_mutation_counter() {
    let env = Rc::new(RefCell::new(Environment::new()));

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
    assert_eq!(run(input, Rc::clone(&env)), RuntimeValue::Number(7.0));
}
