mod expr;
mod parser;
mod scanner;
mod tokentype;

use parser::Parser;
use scanner::Scanner;

fn main() {
    let mut args = std::env::args();
    if args.len() > 2 {
        println!("Usage: rslox [script]");
        return;
    } else if args.len() == 2 {
        run_file(args.nth(1).unwrap());
    } else {
        run_prompt();
    }
}

fn run_file(path_arg: String) {
    use std::convert::TryInto;
    let bytestring = std::fs::read(path_arg)
        .expect("File not found")
        .try_into()
        .expect("File not utf-8 encoded");
    run(bytestring);
}

fn run_prompt() {
    use std::io::Write;
    let stdin = std::io::stdin();
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        if let Ok(n_read) = stdin.read_line(&mut line) {
            if n_read == 0 {
                break;
            }
            run(line);
        }
    }
}

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    match scanner.scan_tokens() {
        Ok(tokens) => {
            let parser = Parser::new(tokens);
            match parser.parse() {
                Ok(expression) => {
                    println!("{}", expression);
                }
                Err(errors) => {
                    for error in errors {
                        println!("{}", error);
                    }
                }
            }
        }
        Err(errors) => {
            for error in errors {
                println!("{}", error);
            }
        }
    }
}
