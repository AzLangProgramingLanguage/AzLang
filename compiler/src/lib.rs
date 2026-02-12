mod cleaner;
use crate::{builder::build, cleaner::clean_ast, errors::CompilerError};
use parser::Parser;
use std::{env, fs, path::Path};

mod builder;
mod errors;

pub fn compiler(path: &str) -> Result<(), CompilerError> {
    // 1. Dosyayı oku ve Transpile et
    let sdk = file_system::read_file(path)?;
    let tokens = tokenizer::Lexer::new(&sdk)
        .tokenize()
        .map_err(|err| CompilerError::Lexer(err))?;

    let mut parser = Parser::new(tokens);
    let mut parsed_program = parser.parse().map_err(|err| CompilerError::Parser(err))?;

    let mut validator = validator::ValidatorContext::new();
    validator.validate(&mut parsed_program)?;
    clean_ast(&mut parsed_program, &validator);

    let mut ctx = transpiler::TranspileContext::new();
    let code = ctx.transpile(parsed_program);

    let bin_path = Path::new("bin");
    if !bin_path.exists() {
        fs::create_dir(bin_path)
            .map_err(|_| CompilerError::FileError("bin klasörü oluşturulamadı".to_string()))?;
    }

    let deps_src = Path::new("./dependencies");
    let deps_dest = bin_path.join("dependencies");
    if deps_src.exists() {
        copy_dir_all(deps_src, &deps_dest)
            .map_err(|_| CompilerError::FileError("Bağımlılıklar kopyalanamadı".to_string()))?;
    }

    // 4. Çıktı dosyalarını 'bin' içerisine yaz
    let output_zig = bin_path.join("azlang_output.zig");

    file_system::write_file(&output_zig, &code)?;

    // 5. Build işlemini başlat
    build(output_zig.to_str().unwrap(), path)?;

    Ok(())
}

/// Klasörü içindekilerle birlikte kopyalamak için yardımcı fonksiyon
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
