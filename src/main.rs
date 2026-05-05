use std::env;
use std::fs;
use std::process;

fn main() {
    let mut args = env::args().skip(1);
    let Some(first_arg) = args.next() else {
        eprintln!("Usage: cust [--tokens|--ast|--max-steps N] <file.c>");
        process::exit(64);
    };

    if first_arg == "--version" {
        println!("cust {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    let (mode, path) = if first_arg == "--tokens" {
        let Some(path) = args.next() else {
            eprintln!("Usage: cust [--tokens|--ast|--max-steps N] <file.c>");
            process::exit(64);
        };
        (Mode::Tokens, path)
    } else if first_arg == "--ast" {
        let Some(path) = args.next() else {
            eprintln!("Usage: cust [--tokens|--ast|--max-steps N] <file.c>");
            process::exit(64);
        };
        (Mode::Ast, path)
    } else if first_arg == "--max-steps" {
        let Some(limit) = args.next() else {
            eprintln!("cust: --max-steps requires a positive integer");
            process::exit(64);
        };
        let Ok(max_loop_iterations) = limit.parse::<usize>() else {
            eprintln!("cust: --max-steps requires a positive integer");
            process::exit(64);
        };
        if max_loop_iterations == 0 {
            eprintln!("cust: --max-steps requires a positive integer");
            process::exit(64);
        }
        let Some(path) = args.next() else {
            eprintln!("Usage: cust [--tokens|--ast|--max-steps N] <file.c>");
            process::exit(64);
        };
        (
            Mode::Run {
                max_loop_iterations: Some(max_loop_iterations),
            },
            path,
        )
    } else {
        (
            Mode::Run {
                max_loop_iterations: None,
            },
            first_arg,
        )
    };

    if args.next().is_some() {
        eprintln!("Usage: cust [--tokens|--ast|--max-steps N] <file.c>");
        process::exit(64);
    }

    let source = match fs::read_to_string(&path) {
        Ok(source) => source,
        Err(err) => {
            eprintln!("cust: failed to read {path}: {err}");
            process::exit(66);
        }
    };

    match mode.execute(&source) {
        Ok(output) => {
            print!("{output}");
        }
        Err(err) => {
            eprintln!("cust: {err}");
            process::exit(1);
        }
    }
}

enum Mode {
    Run { max_loop_iterations: Option<usize> },
    Tokens,
    Ast,
}

impl Mode {
    fn execute(&self, source: &str) -> cust::CustResult<String> {
        match self {
            Self::Run {
                max_loop_iterations,
            } => cust::interpret_with_options(
                source,
                cust::InterpretOptions {
                    max_loop_iterations: *max_loop_iterations,
                },
            )
            .map(|value| format!("{value}\n")),
            Self::Tokens => cust::format_tokens(source),
            Self::Ast => cust::format_ast(source),
        }
    }
}
