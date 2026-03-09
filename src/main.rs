use std::fs;
use std::io::{self, Write};

use mathfp::{execute_env, interpreter, runtime};

fn usage() {
    println!("Usage: mathfp [file_name]");
}

fn run_file(file_name: &str) -> Result<(), String> {
    let contents = fs::read_to_string(file_name)
        .map_err(|e| format!("Could not read file {file_name}: {e}"))?;

    let interpreter = interpreter::Interpreter::new();
    let _ = execute_env(&contents, &interpreter).map_err(|e| eprintln!("{e}"));

    Ok(())
}

fn run_repl() -> Result<(), String> {
    let interpreter = interpreter::Interpreter::new();

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
                match execute_env(&input, &interpreter) {
                    Ok(value) => {
                        if value != runtime::RuntimeValue::Nil {
                            println!("{value}")
                        }
                    }
                    Err(e) => eprintln!("{e}"),
                };
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
