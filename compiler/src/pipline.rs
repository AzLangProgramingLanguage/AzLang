use parser::ParsedProgram;
use validator::{ValidatedProgram, Validator};

use crate::errors::CompilerError;

pub struct CompilerPipeline<T> {
    value: Result<T, CompilerError>,
}
// impl CompilerPipeline<ParsedProgram> {
//     pub fn validate(self) -> CompilerPipeline<ValidatedProgram> {
//         CompilerPipeline {
//             value: self
//                 .value
//                 .and_then(|program| Validator::default().validate(program)),
//         }
//     }
// }
// impl CompilerPipeline<Vec<Ast>> { }
// impl CompilerPipeline<(Context, Program)> {
//     pub fn transpile(self) -> CompilerPipeline<String> {
//         CompilerPipeline {
//             value: self
//                 .value
//                 .map(|(_, program)| Transpiler::default().transpile(program)),
//         }
//     }
// }
// pub fn parser(source: String) -> CompilerPipeline<ParsedProgram> {
//     CompilerPipeline {
//         value: parse(source),
//     }
// }
