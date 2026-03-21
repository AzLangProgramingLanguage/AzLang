
use std::io::{self, Write};
use file_system;
use validator::Validator;
mod errors;
mod runner;
use crate::{errors::InterPreterError, runner::Runner};
use parser::parser;
pub use validator::validate::validate_expr;

pub fn interpreter_file(path: &str)-> Result<(),InterPreterError> {
    let sdk = file_system::read_file(path)?;
    let mut lexer = tokenizer::Lexer::new(&sdk);
     let mut tokens = lexer.tokenize()?;
     let mut parsed_program = parser(&mut tokens)?;
    
     let mut validator = validator::Validator::new();
     validator.validate(&mut parsed_program)?;
     let mut runner = Runner::new();
     for expr in parsed_program.expressions {
         runner.run(expr);
     }
     Ok(())
}

pub fn interpreter_run_repl()->Result<(),InterPreterError> {

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
            return Ok(());
        }
        let mut lexer = tokenizer::Lexer::new(&input);
         let mut tokens = lexer.tokenize()?;
        let expressions = {
             let mut parsed_program = parser(&mut tokens)?;
            validator.validate(&mut parsed_program)?;
            parsed_program.expressions
        };
        for expr in expressions {
            runner.run(expr);
        }
    } 
}
