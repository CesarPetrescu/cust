use std::env;
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

    match mode.execute(&path) {
        Ok(output) => {
            print!("{output}");
        }
        Err(err) => {
            eprintln!("cust: {err}");
            process::exit(if err.is_io_error() { 66 } else { 1 });
        }
    }
}

enum Mode {
    Run { max_loop_iterations: Option<usize> },
    Tokens,
    Ast,
}

impl Mode {
    fn execute(&self, path: &str) -> cust::CustResult<String> {
        match self {
            Self::Run {
                max_loop_iterations,
            } => cust::interpret_file_with_options(
                path,
                cust::InterpretOptions {
                    max_loop_iterations: *max_loop_iterations,
                },
            )
            .map(|value| format!("{value}\n")),
            Self::Tokens => cust::format_file_tokens(path),
            Self::Ast => cust::format_file_ast(path),
        }
    }
}
