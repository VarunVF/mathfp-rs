use mathfp::eval;
use mathfp::parser::Parser;
use mathfp::runtime::{Environment, RuntimeValue};
use mathfp::token::Scanner;

fn run(input: &str, env: &mut Environment) -> RuntimeValue {
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
    let mut env = Environment::new();

    // dangling else
    let input = "if true then if false then 1 else 2";
    assert_eq!(run(input, &mut env), RuntimeValue::Number(2.0));

    // dangling else
    let input = "if false then if false then 1 else 2";
    assert_eq!(run(input, &mut env), RuntimeValue::Nil);

    // expression nesting
    let input2 = "x := 10; 5 + (if x then 10 else 0)";
    assert_eq!(run(input2, &mut env), RuntimeValue::Number(15.0));
}
