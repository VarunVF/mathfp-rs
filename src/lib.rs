pub mod ast;
pub mod eval;
pub mod parser;
pub mod runtime;
pub mod token;

pub fn execute(
    input: &str,
    env: &mut runtime::Environment,
) -> Result<runtime::RuntimeValue, String> {
    let tokens = token::Scanner::new(input)
        .scan()
        .map_err(|errors| token::Scanner::report(&errors))?;

    let expr = parser::Parser::new(tokens)
        .parse()
        .map_err(|errors| parser::Parser::report(&errors))?;

    eval::evaluate(expr, env)
}
