use std::fs;
use std::io::{self, Write};

use mathfp::{execute_env, interpreter, runtime};

fn run_file(file_name: &str, args: &[String]) -> Result<(), String> {
    let contents = fs::read_to_string(file_name)
        .map_err(|e| format!("Could not read file {file_name}: {e}"))?;

    let interpreter = interpreter::Interpreter::new(args);
    let _ = execute_env(&contents, &interpreter).map_err(|e| eprintln!("{e}"));

    Ok(())
}

fn run_repl() -> Result<(), String> {
    let interpreter = interpreter::Interpreter::default();

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
            0 => {
                // EOF
                println!();
                return Ok(());
            }
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
    match argv.as_slice() {
        [] => Err("Recieved no arguments.".to_string()),
        [_program_name] => run_repl(),
        [_program_name, file_path] => run_file(file_path, &[]),
        [_program_name, file_path, script_args @ ..] => run_file(file_path, script_args),
    }
}
