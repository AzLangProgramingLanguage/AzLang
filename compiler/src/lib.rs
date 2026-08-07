use parser::parser;
mod errors;
#[cfg(test)]
mod tests;

use crate::errors::CompilerError;

pub fn compiler(path: &str) -> Result<(), CompilerError> {
    let source = file_system::read_file(path)?;

    let parsed_program = parser(source)?;

    let validator = validator::Validator::default();
    let (_, program) = validator.validate(parsed_program)?;

    // TODO: Backend hələ təsdiqlənməyib. Kompilasyon ardıcıllığı buradan davam edir.
    let _ = program;

    Ok(())
}