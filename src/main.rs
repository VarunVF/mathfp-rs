use std::io::{self, Write};
use std::{env::args, fs};

use mathfp::{execute_env, interpreter::Interpreter, runtime::RuntimeValue};

fn run_file(args: &[String]) -> Result<(), String> {
    let file_name = args.first().ok_or("No script file was provided to run")?;
    let contents = fs::read_to_string(file_name)
        .map_err(|e| format!("Could not read file '{file_name}': {e}"))?;

    let interpreter = Interpreter::new(args);
    let _ = execute_env(&contents, &interpreter).map_err(|e| eprintln!("{e}"));

    Ok(())
}

fn run_repl() -> Result<(), String> {
    let interpreter = Interpreter::default();

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
                        if value != RuntimeValue::Nil {
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
    let argv: Vec<String> = args().collect();
    match argv.as_slice() {
        [] | [_] => run_repl(),
        [_, args @ ..] => run_file(args),
    }
}
