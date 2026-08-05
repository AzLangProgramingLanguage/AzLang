use crate::{errors::CompilerError, parser};
use transpiler::TranspileContext;

#[test]
fn compiler_ffi_test() -> Result<(), CompilerError> {
    let parsed_program = parser(
        r#"@external("../build/printlib.so", "printValue")\nfunksiya print(sabit hərşey val): heçnə\nprint(1)"#.to_string(),
    )?;

    let validator = validator::Validator::default();
    let (_, program) = validator.validate(parsed_program)?;

    let mut ctx = TranspileContext::default();
    let code = ctx.transpile(program);
    assert!(code.contains("const a: f64 = 5.1;"));
    Ok(())
}
