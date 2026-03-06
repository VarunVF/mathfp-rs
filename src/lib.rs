pub mod ast;
pub mod builtins;
pub mod eval;
pub mod parser;
pub mod runtime;
pub mod token;

pub fn execute_env(
    input: &str,
    env: std::rc::Rc<std::cell::RefCell<runtime::Environment>>,
) -> Result<runtime::RuntimeValue, String> {
    let tokens = token::Scanner::new(input)
        .scan()
        .map_err(|errors| token::Scanner::report(&errors))?;

    let expr = parser::Parser::new(tokens)
        .parse()
        .map_err(|errors| parser::Parser::report(&errors))?;

    eval::evaluate(expr, env)
}

pub fn execute(input: &str) -> Result<runtime::RuntimeValue, String> {
    use runtime::Environment;
    use std::cell::RefCell;
    use std::rc::Rc;

    let env: Rc<RefCell<runtime::Environment>> = Rc::new(RefCell::new(Environment::new()));
    execute_env(input, env)
}

pub fn execute_env_or_panic(
    input: &str,
    env: std::rc::Rc<std::cell::RefCell<runtime::Environment>>,
) -> runtime::RuntimeValue {
    execute_env(input, env).expect("Evaluation should run correctly")
}

pub fn execute_or_panic(input: &str) -> runtime::RuntimeValue {
    execute(input).expect("Evaluation should run correctly")
}
