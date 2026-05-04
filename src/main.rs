use std::env;
use std::fs;
use std::process;

fn main() {
    let mut args = env::args().skip(1);
    let Some(first_arg) = args.next() else {
        eprintln!("Usage: cust [--tokens] <file.c>");
        process::exit(64);
    };

    if first_arg == "--version" {
        println!("cust {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    let (mode, path) = if first_arg == "--tokens" {
        let Some(path) = args.next() else {
            eprintln!("Usage: cust [--tokens] <file.c>");
            process::exit(64);
        };
        (Mode::Tokens, path)
    } else {
        (Mode::Run, first_arg)
    };

    if args.next().is_some() {
        eprintln!("Usage: cust [--tokens] <file.c>");
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
    Run,
    Tokens,
}

impl Mode {
    fn execute(&self, source: &str) -> cust::CustResult<String> {
        match self {
            Self::Run => cust::interpret(source).map(|value| format!("{value}\n")),
            Self::Tokens => cust::format_tokens(source),
        }
    }
}
