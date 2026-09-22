use std::process::Command;

use parser::ParsedProgram;
use transpiler::Transpiler;
use validator::{ValidatedProgram, Validator, errors::ValidatorError};
pub fn executer(static_libs: Vec<String>) -> std::process::Output {
    Command::new("qbe")
        .args(["-o", "main.s", "main.ssa"])
        .status()
        .expect("Error");
    Command::new("as")
        .args(["main.s", "-o", "main.o"])
        .status()
        .expect("Error");
    Command::new("ld.lld")
        .args(static_libs)
        .args(["main.o", "-o", "app"])
        .status()
        .expect("Linker Error");
    Command::new("./app").output().expect("Çalışdırılmadı")
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
