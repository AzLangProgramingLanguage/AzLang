mod cleaner;
use parser::parser;
use validator::Validator;

use crate::{builder::build, cleaner::clean_ast, errors::CompilerError};
use std::{env, fs, path::Path};
use logging::translator_log;
mod builder;
mod errors;

pub fn compiler(path: &str) {

 let sdk = file_system::read_file(path).unwrap_or_else(|err| {
        println!("\x1b[31m[Böyük Qardaş]:\x1b[0m {}", err.kind);
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

    clean_ast(&mut parsed_program, &validator);

    let mut ctx = transpiler::TranspileContext::new();
    let code = ctx.transpile(parsed_program);

    let bin_path = Path::new("bin");
    if !bin_path.exists() {
        fs::create_dir(bin_path)
            .unwrap_or_else(|err| {
                println!("\x1b[31m[Böyük Qardaş]:\x1b[0m {}", err);
                std::process::exit(1);
            });
    }

    let deps_src = Path::new("./dependencies");
    let deps_dest = bin_path.join("dependencies");
    if deps_src.exists() {
        copy_dir_all(deps_src, &deps_dest)
            .unwrap_or_else(|err| {
                println!("\x1b[31m[Böyük Qardaş]:\x1b[0m {}", err);
                std::process::exit(1);
            })
    }

    let output_zig = bin_path.join("azlang_output.zig");

    file_system::write_file(&output_zig, &code).unwrap_or_else(|err| {
        println!("\x1b[31m[Böyük Qardaş]:\x1b[0m {}", err.kind);
        std::process::exit(err.code());
    });

    build(output_zig.to_str().unwrap(), path).unwrap_or_else(|err| {
        translator_log(&err.to_string());
    });

}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
