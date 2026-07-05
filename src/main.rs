mod scanner;
mod tokentype;
use scanner::Scanner;
fn main() {
    let mut args = std::env::args();
    let mut lox = Lox::new();
    if args.len() > 2 {
        println!("Usage: rslox [script]");
        return;
    } else if args.len() == 2 {
        lox.run_file(args.nth(1).unwrap());
    } else {
        lox.run_prompt();
    }
}

struct Lox {
    hadError: bool,
}

impl Lox {
    fn new() -> Self {
        Self { hadError: false }
    }
    fn run_file(&mut self, path_arg: String) {
        use std::convert::TryInto;
        let bytestring = std::fs::read(path_arg)
            .expect("File not found")
            .try_into()
            .expect("File not utf-8 encoded");
        self.run(bytestring);
    }

    fn run_prompt(&mut self) {
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
                self.run(line);
                self.hadError = false;
            }
        }
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        for token in tokens {
            println!("{}", token);
        }
    }

    fn error(&mut self, line: i32, message: String) {
        self.report(line, String::new(), message);
    }

    fn report(&mut self, line: i32, position: String, message: String) {
        println!("[line {} ] Error {} : {}", line, position, message);
        self.hadError = true;
    }
}
