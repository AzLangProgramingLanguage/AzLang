use std::path::{Path, PathBuf};

use file_system::write_file;
use parser::parser;
use transpiler::Transpiler;
use validator::Validator;
mod errors;
mod pipline;

use crate::{errors::CompilerError::self, pipline::assemble};
const STANDARTLIB: &str = "sdk/src/lib.az";

pub struct Compiled {
    pub validator: Validator,
    pub code: Vec<String>,
    pub data: Vec<String>,
    pub called: std::collections::HashSet<String>,
}

pub fn compiler(path: &str) -> Result<(), CompilerError> {
    let file_name = Path::new(path)
        .file_stem()
        .and_then(|name| name.to_str())
        .expect("Your file name isn't valid. Please write utf-8 standarts");

    let compiled = compile_obj(path, true, file_name)?;

    let mut asm_path = PathBuf::from(path);
    asm_path.set_file_name(file_name);
    asm_path.set_extension("asm");
    write_file(&asm_path, build_asm(&compiled))?;
    assemble(&asm_path, &PathBuf::from(file_name))?;
    Ok(())
}

fn build_asm(compiled: &Compiled) -> String {
    let mut asm = String::from("format ELF64 executable\nentry _start\n\nsegment readable executable\n\n");

    let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for (op, file) in &compiled.validator.link_files {
        if !compiled.called.contains(op) {
            continue;
        }
        if !seen.insert(file.as_str()) {
            continue;
        }
        let absolute = std::fs::canonicalize(file)
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_else(|_| file.clone());
        asm.push_str(&format!("include \"{absolute}\"\n"));
    }
    if !seen.is_empty() {
        asm.push('\n');
    }

    for code in &compiled.code {
        asm.push_str(code);
        asm.push('\n');
    }
    if !compiled.data.is_empty() {
        asm.push_str(&compiled.data.join("\n"));
        asm.push('\n');
    }
    asm
}

pub fn compile_obj(
    path: &str,
    is_start: bool,
    file_name: &str,
) -> Result<Compiled, CompilerError> {
    println!("{path}");
    let module_source = file_system::read_file(path)?;
    let parsed = parser(module_source)?;

    let mut validator = Validator::default();
    let mut code: Vec<String> = Vec::new();
    let mut data: Vec<String> = Vec::new();
    let mut called: std::collections::HashSet<String> = std::collections::HashSet::new();

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

        validator.functions.extend(compiled_context.validator.functions);
        validator
            .variables
            .extend(compiled_context.validator.variables);
        validator
            .constvars
            .extend(compiled_context.validator.constvars);
        validator
            .link_files
            .extend(compiled_context.validator.link_files);
        code.extend(compiled_context.code);
        data.extend(compiled_context.data);
        called.extend(compiled_context.called);
    }
    let mut validated_program = validator.validate(parsed.ast)?;
    validated_program.variables = validator.constvars.clone();

    let externals: std::collections::HashSet<String> = validator
        .link_files
        .iter()
        .map(|(op, _)| op.clone())
        .collect();

    let mut transpiler = Transpiler::default();
    let output = transpiler.transpile(file_name, validated_program, is_start, externals);

    code.push(output.code);
    if !output.data.is_empty() {
        data.push(output.data);
    }
    called.extend(output.called);

    Ok(Compiled {
        validator,
        code,
        data,
        called,
    })
}

#[cfg(test)]
mod tests;
