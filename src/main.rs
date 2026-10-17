use std::fs;
use std::io::{self, Write};

use mathfp::latex::Transpiler;
use mathfp::{execute_env, interpreter::Interpreter, runtime::RuntimeValue};

use clap::{self, Parser};

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Script file to read program from
    script: Option<String>,

    /// Transpile expressions to LaTeX code
    #[arg(short, long)]
    latex: bool,

    /// Arguments passed to program
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    script_args: Vec<String>,
}

fn run_file(file_name: &str, args: &Cli) -> Result<(), String> {
    let contents = fs::read_to_string(file_name)
        .map_err(|e| format!("Could not read file '{file_name}': {e}"))?;

    if args.latex {
        match Transpiler::new(true).transpile(&contents) {
            Ok(latex) => println!("{latex}"),
            Err(e) => eprintln!("{e}"),
        }
    } else {
        let interpreter = Interpreter::new(&args.script_args);
        let _ = execute_env(&contents, &interpreter).map_err(|e| eprintln!("{e}"));
    }

    Ok(())
}

fn run_repl(args: &Cli) -> Result<(), String> {
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

        if bytes_read == 0 {
            // EOF
            println!();
            return Ok(());
        } else if args.latex {
            match Transpiler::new(false).transpile(&input) {
                Ok(latex) => println!("{latex}"),
                Err(e) => eprintln!("{e}"),
            }
        } else {
            match execute_env(&input, &interpreter) {
                Ok(RuntimeValue::Nil) => (),
                Ok(value) => println!("{value}"),
                Err(e) => println!("{e}"),
            }
        }
    }
}

fn main() -> Result<(), String> {
    let args = Cli::parse();
    if let Some(ref file_name) = args.script {
        run_file(file_name, &args)
    } else {
        run_repl(&args)
    }
}
