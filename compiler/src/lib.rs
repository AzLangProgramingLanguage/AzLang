use std::path::{Path, PathBuf};

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
const STANDARTLIB: &str = "sdk/src/lib.az";

pub fn compiler(path: &str) -> Result<(), CompilerError> {
    let file_name = Path::new(path)
        .file_stem()
        .and_then(|name| name.to_str())
        .expect("Your file name isn't valid. Please write utf-8 standarts");

    let context = compile_obj(path, true, &file_name)?;
    executer(file_name, context.link_files);
    Ok(())
}
pub fn compile_obj(
    path: &str,
    is_start: bool,
    file_name: &str,
) -> Result<Validator, CompilerError> {
    println!("{path}");
    let module_source = file_system::read_file(path)?;
    let parsed = parser(module_source)?;

    let mut validator = Validator::default();

    for module in parsed.modules {
        let mut obj_path = PathBuf::from(path);
        if module == "lib" {
            obj_path = PathBuf::from(STANDARTLIB);
        } else {
            obj_path.set_file_name(format!("{module}.az"));
        }

        let compiled_context = compile_obj(
            obj_path
                .to_str()
                .expect("Your file name isn't valid. Please use UTF-8 standards"),
            false,
            &module,
        )?;

        validator.functions.extend(compiled_context.functions);
        validator.variables.extend(compiled_context.variables);
        validator.constvars.extend(compiled_context.constvars);
        validator.link_files.extend(compiled_context.link_files);
    }
    let mut validated_program = validator.validate(parsed.ast)?;
    let mut transpiler = Transpiler::default();
    validated_program.variables = validator.constvars.clone();
    let transpiled_code;
    if is_start {
        transpiled_code = transpiler.transpile_start(validated_program);
    } else {
        transpiled_code = transpiler.transpile_module(&file_name, validated_program);
    }

    let mut path = PathBuf::from(path);
    path.set_file_name(file_name);
    path.set_extension("il");

    write_file(&path, transpiled_code)?;
    let obj = compile_to_obj(path);
    validator.link_files.push(obj.to_str().unwrap().to_string());

    return Ok(validator);
}

#[cfg(test)]
mod tests;
