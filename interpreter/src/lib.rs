use std::io::{self, Write};
use validator::{Validator, ast::Ast};
mod errors;
mod runner;
use crate::{errors::InterPreterError, runner::Runner};
use parser::{ast::Parameter, parser, shared_ast::Type};
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    body: Vec<Ast>,
    params: Vec<Parameter>,
    return_type: Type,
}
pub fn interpreter_file(path: &str) -> Result<(), InterPreterError> {
    let sdk = file_system::read_file(path)?;
    let parsed_program = parser(sdk)?;
    let validator = validator::Validator::default();
    let result = validator.validate(parsed_program)?;
    let mut runner = Runner::new();
    for function in result.1.functions {
        runner.functions.insert(
            function.name,
            Function {
                body: function.body,
                params: function.params,
                return_type: function.return_typ,
            },
        );
    }
    for stmt in result.1.expressions {
        runner.run(stmt);
    }
    Ok(())
}

pub fn interpreter_run_repl() -> Result<(), InterPreterError> {
    println!("AzLang REPL başladı. Çıxmaq üçün 'exit' yaz.");
    let mut runner = Runner::new();
    let mut validator = Validator::default();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();

        if trimmed == "exit" {
            return Ok(());
        }
        let parsed_program = parser(input)?;
        let (new_validator, program) = validator.validate(parsed_program)?;
        validator = new_validator;
        for function in program.functions {
            runner.functions.insert(
                function.name,
                Function {
                    body: function.body,
                    params: function.params,
                    return_type: function.return_typ,
                },
            );
        }
        for expr in program.expressions {
            runner.run(expr);
        }
    }
}
