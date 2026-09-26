use std::path::PathBuf;

use file_system::write_file;
use parser::parser;
use transpiler::Transpiler;
use validator::Validator;
mod errors;
mod pipline;

use crate::{
    errors::CompilerError::{self},
    pipline::{compile_to_obj, executer},
};

pub fn compiler(path: &str) -> Result<(), CompilerError> {
    let source = file_system::read_file(path)?;
    let parsed_program = parser(source)?;
    let mut validator = Validator::default();
    for module in parsed_program.modules {
        let compiled_code = compile_obj(path, module)?;
        validator.functions.extend(compiled_code.functions);
        validator.variables.extend(compiled_code.variables);
        validator.constvars.extend(compiled_code.constvars);
        validator.link_files.extend(compiled_code.link_files);
    }

    let mut main_file = PathBuf::from(path);
    main_file.set_extension("il");
    let mut transpiler = Transpiler::default();
    let mut validated = validator.validate(parsed_program.ast)?;
    validated.variables = validator.constvars;

    let transpiled_code = transpiler.transpile_start(validated);

    write_file(&main_file, transpiled_code)?;
    executer(main_file, validator.link_files);

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
        let mut obj_path = PathBuf::from(path);
        obj_path.set_file_name(format!("{module}.o"));
        validator
            .link_files
            .push(obj_path.to_str().unwrap().to_string());
        let compiled_context = compile_obj(path, module)?;

        validator.functions.extend(compiled_context.functions);
        validator.variables.extend(compiled_context.variables);
        validator.constvars.extend(compiled_context.constvars);
        validator.link_files.extend(compiled_context.link_files);
    }

    let mut validated_program = validator.validate(parsed.ast)?;
    let mut transpiler = Transpiler::default();
    validated_program.variables = validator.constvars.clone();

    let transpiled_code = transpiler.transpile_module(&module, validated_program);
    let mut path = PathBuf::from(path);
    path.set_file_name(module);
    path.set_extension("il");

    write_file(&path, transpiled_code)?;
    let obj = compile_to_obj(path);
    validator.link_files.push(obj.to_str().unwrap().to_string());

    return Ok(validator);
}

#[cfg(test)]
mod tests;
