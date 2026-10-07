pub mod ast;
pub mod builtins;
pub mod interpreter;
pub mod parser;
pub mod runtime;
pub mod scanner;
pub mod token;

pub fn execute(input: &str, args: &[String]) -> Result<runtime::RuntimeValue, String> {
    execute_env(input, &interpreter::Interpreter::new(args))
}

pub fn execute_or_panic(input: &str, args: &[String]) -> runtime::RuntimeValue {
    execute_env_or_panic(input, &interpreter::Interpreter::new(args))
}

pub fn execute_env(
    input: &str,
    interpreter: &interpreter::Interpreter,
) -> Result<runtime::RuntimeValue, String> {
    let tokens = scanner::Scanner::new(input)
        .scan()
        .map_err(|errors| scanner::Scanner::report(&errors))?;

    let expr = parser::Parser::new(tokens)
        .parse()
        .map_err(|errors| parser::Parser::report(&errors))?;

    interpreter
        .interpret(&expr)
        .map_err(|error| interpreter::Interpreter::report(&error))
}

pub fn execute_env_or_panic(
    input: &str,
    interpreter: &interpreter::Interpreter,
) -> runtime::RuntimeValue {
    execute_env(input, interpreter).expect("Evaluation should run correctly")
}
