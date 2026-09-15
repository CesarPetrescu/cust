use std::env;
use std::process;

const USAGE: &str = "Usage: cust [--tokens|--ast|--max-steps N] <file.c>";
const HELP: &str = concat!(
    "Usage: cust [--tokens|--ast|--max-steps N] <file.c>\n",
    "\n",
    "Interpret a supported C source file.\n",
    "\n",
    "Modes:\n",
    "  <file.c>                     Interpret the source file.\n",
    "  --tokens <file.c>            Print lexer tokens without interpreting.\n",
    "  --ast <file.c>               Print the parsed AST without interpreting.\n",
    "  --max-steps N <file.c>       Limit total loop iterations to positive N.\n",
    "\n",
    "Options:\n",
    "  -h, --help                   Print this help message and exit.\n",
    "  --version                    Print the Cust version and exit.\n",
);

fn main() {
    let mut args = env::args().skip(1);
    let Some(first_arg) = args.next() else {
        eprintln!("{USAGE}");
        process::exit(64);
    };

    if first_arg == "--help" || first_arg == "-h" {
        print!("{HELP}");
        return;
    }

    if first_arg == "--version" {
        println!("cust {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    let (mode, path, path_is_delimited) = if first_arg == "--" {
        let Some(path) = args.next() else {
            eprintln!("{USAGE}");
            process::exit(64);
        };
        (
            Mode::Run {
                max_loop_iterations: None,
            },
            path,
            true,
        )
    } else if first_arg == "--tokens" {
        let Some((path, path_is_delimited)) = next_source_operand(&mut args) else {
            eprintln!("{USAGE}");
            process::exit(64);
        };
        (Mode::Tokens, path, path_is_delimited)
    } else if first_arg == "--ast" {
        let Some((path, path_is_delimited)) = next_source_operand(&mut args) else {
            eprintln!("{USAGE}");
            process::exit(64);
        };
        (Mode::Ast, path, path_is_delimited)
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
        let Some((path, path_is_delimited)) = next_source_operand(&mut args) else {
            eprintln!("{USAGE}");
            process::exit(64);
        };
        (
            Mode::Run {
                max_loop_iterations: Some(max_loop_iterations),
            },
            path,
            path_is_delimited,
        )
    } else if first_arg.starts_with('-') {
        eprintln!("cust: unknown option '{first_arg}'");
        eprintln!("{USAGE}");
        process::exit(64);
    } else {
        (
            Mode::Run {
                max_loop_iterations: None,
            },
            first_arg,
            false,
        )
    };

    if !path_is_delimited && path.starts_with('-') {
        eprintln!("cust: unknown option '{path}'");
        eprintln!("{USAGE}");
        process::exit(64);
    }

    if args.next().is_some() {
        eprintln!("{USAGE}");
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

fn next_source_operand(args: &mut impl Iterator<Item = String>) -> Option<(String, bool)> {
    let operand = args.next()?;
    if operand == "--" {
        args.next().map(|path| (path, true))
    } else {
        Some((operand, false))
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
