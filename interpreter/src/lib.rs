use std::io::{self, Write};
use validator::{Validator, ast::Ast};
mod errors;
mod runner;
use crate::{errors::InterPreterError, runner::Runner};
use parser::{ast::Parameter, parser, shared_ast::Type};
use runner::ExternalFunction;
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

    // Resolve the directory of the source file so that relative library paths
    // (e.g. "printlib.so" declared inside sdk/data_structures.az) are found
    // next to the .az file rather than requiring a system-wide install.
    let source_dir = std::path::Path::new(path)
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::path::PathBuf::from("."));

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
    for ext in result.1.external_functions {
        // If the library path is relative, resolve it against the source file's dir.
        let library_path = {
            let p = std::path::Path::new(&ext.library);
            if p.is_absolute() {
                ext.library.clone()
            } else {
                source_dir
                    .join(p)
                    .to_string_lossy()
                    .into_owned()
            }
        };
        runner.external_functions.insert(
            ext.name.clone(),
            ExternalFunction {
                library: library_path,
                symbol: ext.symbol,
                params: ext.params,
                return_type: ext.return_typ,
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
        for ext in program.external_functions {
            runner.external_functions.insert(
                ext.name.clone(),
                ExternalFunction {
                    library: ext.library,
                    symbol: ext.symbol,
                    params: ext.params,
                    return_type: ext.return_typ,
                },
            );
        }
        for expr in program.expressions {
            runner.run(expr);
        }
    }
}
