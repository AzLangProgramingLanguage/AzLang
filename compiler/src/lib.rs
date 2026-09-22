use std::path::PathBuf;

use file_system::write_file;
use parser::parser;
use transpiler::Transpiler;
use validator::Validator;
mod errors;
mod pipline;

use crate::{
    errors::CompilerError::{self},
    pipline::{CompilerPipline, executer},
};

pub fn compiler(path: &str) -> Result<(), CompilerError> {
    let source = file_system::read_file(path)?;
    let parsed_program = parser(source)?;
    let validator = Validator::default();
    for module in parsed_program.modules {
        compile_obj(path, module)?;
    }
    // let (context, transpiled_code) = CompilerPipline(parser(source)?).validate()?.transpile();

    // let main_file = PathBuf::from("main.ssa");
    // write_file(&main_file, transpiled_code)?;

    executer(context.link_files);

    Ok(())
}
pub fn compile_obj(path: &str, module: String) -> Result<Validator, CompilerError> {
    let module_source = file_system::read_file(
        PathBuf::from(path)
            .parent()
            .expect("Doesn't have any parent")
            .join(format!("{module}.az"))
            .to_str()
            .expect("File not found"),
    )?;

    let parsed = parser(module_source)?;

    let mut validator = Validator::default();

    for module in parsed.modules {
        let compiled_context = compile_obj(path, module)?;
        validator.functions.extend(compiled_context.functions);
        validator.variables.extend(compiled_context.variables);
    }

    let validated_program = validator.validate(parsed.ast)?;
    let mut transpiler = Transpiler::default();
    let transpiled_code = transpiler.transpile(validated_program.1);
    write_file(path, content)
    Ok(validator)
}

#[cfg(test)]
mod tests;
