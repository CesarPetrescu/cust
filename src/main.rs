use std::env;
use std::fs;
use std::process;

fn main() {
    let mut args = env::args().skip(1);
    let Some(path) = args.next() else {
        eprintln!("Usage: cust <file.c>");
        process::exit(64);
    };

    if path == "--version" {
        println!("cust {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if args.next().is_some() {
        eprintln!("Usage: cust <file.c>");
        process::exit(64);
    }

    let source = match fs::read_to_string(&path) {
        Ok(source) => source,
        Err(err) => {
            eprintln!("cust: failed to read {path}: {err}");
            process::exit(66);
        }
    };

    match cust::interpret(&source) {
        Ok(value) => {
            println!("{value}");
        }
        Err(err) => {
            eprintln!("cust: {err}");
            process::exit(1);
        }
    }
}
