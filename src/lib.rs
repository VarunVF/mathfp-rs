pub mod ast;
pub mod builtins;
pub mod eval;
pub mod parser;
pub mod runtime;
pub mod token;

pub fn execute(input: &str) -> Result<runtime::RuntimeValue, String> {
    use runtime::Environment;
    use std::cell::RefCell;
    use std::rc::Rc;

    let env: Rc<RefCell<runtime::Environment>> = Rc::new(RefCell::new(Environment::new()));

    let tokens = token::Scanner::new(input)
        .scan()
        .map_err(|errors| token::Scanner::report(&errors))?;

    let expr = parser::Parser::new(tokens)
        .parse()
        .map_err(|errors| parser::Parser::report(&errors))?;

    eval::evaluate(expr, env)
}
