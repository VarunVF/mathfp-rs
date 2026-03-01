mod ast;
mod eval;
mod parser;
mod runtime;
mod token;

use std::fs;
use std::io::{self, Write};

fn usage() {
    println!("Usage: mathfp [file_name]");
}

fn run(source: &str, env: &mut runtime::Environment) -> Result<(), String> {
    let tokens = token::Scanner::new(source)
        .scan()
        .map_err(|errors| token::Scanner::report(&errors))?;

    let program = parser::Parser::new(tokens)
        .parse()
        .map_err(|errors| parser::Parser::report(&errors))?;

    let result = eval::evaluate(program, env)?;
    runtime::display(&result);

    Ok(())
}

fn run_file(file_name: &str) -> Result<(), String> {
    let contents = fs::read_to_string(file_name)
        .map_err(|e| format!("Could not read file {file_name}: {e}"))?;

    let mut env = runtime::Environment::new();
    let _ = run(&contents, &mut env).map_err(|e| eprintln!("{e}"));

    Ok(())
}

fn run_repl() -> Result<(), String> {
    let mut env = runtime::Environment::new();

    loop {
        print!(">>> ");
        io::stdout()
            .flush()
            .map_err(|e| format!("Failed to flush stdout: {e}"))?;

        let mut input = String::new();
        let bytes_read = io::stdin()
            .read_line(&mut input)
            .map_err(|e| format!("Error reading input: {e}"))?;

        match bytes_read {
            0 => return Ok(()), // EOF
            _ => {
                let _ = run(&input, &mut env).map_err(|e| eprintln!("{e}"));
            }
        };
    }
}

fn main() -> Result<(), String> {
    let argv: Vec<String> = std::env::args().collect();
    match argv.len() {
        1 => run_repl(),
        2 => run_file(&argv[1]),
        _ => {
            usage();
            Err("Invalid number of arguments.".to_string())
        }
    }
}
