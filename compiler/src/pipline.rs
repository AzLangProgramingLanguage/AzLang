use std::{ffi::OsString, path::PathBuf, process::Command};

use parser::ParsedProgram;
use transpiler::Transpiler;
use validator::{ValidatedProgram, Validator, errors::ValidatorError};
pub fn executer(program_name: PathBuf, static_libs: Vec<String>) -> std::process::Output {
    let obj = compile_to_obj(program_name);
    Command::new("ld.lld")
        .args(static_libs)
        .arg(obj)
        .args(["-o", "app"])
        .output()
        .expect("Linker Error")
}
pub fn compile_to_obj(module: PathBuf) -> PathBuf {
    let mut s = module.to_path_buf();
    s.set_extension("s");

    let mut il = module.to_path_buf();
    il.set_extension("il");

    let mut o = module;
    o.set_extension("o");

    Command::new("qbe")
        .args(["-o"])
        .arg(&s)
        .arg(&il)
        .status()
        .expect("Error");

    Command::new("as")
        .arg(&s)
        .arg("-o")
        .arg(&o)
        .status()
        .expect("Error");
    o
}
pub struct CompilerPipline<T>(pub T);
// impl CompilerPipline<ParsedProgram> {
//     pub fn validate(self) -> Result<CompilerPipline<ValidatedProgram>, ValidatorError> {
//         let validator = validator::Validator::default();
//         for module in self.0.modules {
//             //TODO: We must finish this stuff.
//         }
//         let validated = validator.validate(self.0.ast)?;
//         Ok(CompilerPipline(validated))
//     }
// }
// impl CompilerPipline<ValidatedProgram> {
//     pub fn transpile(self) -> (validator::Validator, String) {
//         (self.0.0, Transpiler::default().transpile(self.0.1))
//     }
// }
