
use std::io::{self, Write};
use file_system;
use validator::Validator;
mod errors;
mod runner;
use crate::{runner::Runner};
use parser::parser;
pub use validator::validate::validate_expr;

pub fn interpreter_file(path: &str) {
    let sdk = file_system::read_file(path).unwrap_or_else(|err| {
        print!("\x1b[31m[Böyük Qardaş]:\x1b[0m {} ", err.kind);
        println!("\x1b[31m{}\x1b[0m",path);
        std::process::exit(err.code());
    });
    let mut lexer = tokenizer::Lexer::new(&sdk);
     let mut tokens = lexer.tokenize().unwrap_or_else(|err| {
        println!("\x1b[31m[Böyük Qardaş]:\x1b[0m {}", err);
        std::process::exit(1);
    });
     let mut parsed_program = parser(&mut tokens).unwrap_or_else(|err| {
        println!("\x1b[31m[Böyük Qardaş]:\x1b[0m {}", err);
        std::process::exit(1);
    });
    
     let mut validator = validator::Validator::new();
     validator.validate(&mut parsed_program).unwrap_or_else(|err| {
        println!("\x1b[33m[Dəmir Əmi Validator]:\x1b[0m {}", err);
        std::process::exit(1);
     });
     let mut runner = Runner::new();
     for expr in parsed_program.expressions {
         runner.run(expr);
     }
}

pub fn interpreter_run_repl() {

    println!("AzLang REPL başladı. Çıxmaq üçün 'exit' yaz.");

    let mut runner = Runner::new();
    let mut validator = Validator::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();

        if trimmed == "exit" {
            return;
        }
        let mut lexer = tokenizer::Lexer::new(&input);
         let mut tokens = lexer.tokenize().unwrap_or_else(|err| {
        println!("\x1b[31m[Böyük Qardaş]:\x1b[0m {}", err);
        std::process::exit(1);
    });
        let expressions = {
             let mut parsed_program = parser(&mut tokens).unwrap_or_else(|err| {
        println!("\x1b[31m[Böyük Qardaş]:\x1b[0m {}", err);
        std::process::exit(1);
    });
            validator.validate(&mut parsed_program).unwrap_or_else(|err| {
        println!("\x1b[33m[Dəmir Əmi Validator]:\x1b[0m {}", err);
        std::process::exit(1);
     });
            parsed_program.expressions
        };
        for expr in expressions {
            runner.run(expr);
        }
    } 
}
