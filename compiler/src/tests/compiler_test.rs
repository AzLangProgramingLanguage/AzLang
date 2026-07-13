use core::panic;

use crate::{bin_create_dir, builder::build, errors::CompilerError, parser};
use transpiler::TranspileContext;

#[test]
fn compiler_variable_test() -> Result<(), CompilerError> {
    const PATH: &str = "../examples/float.az";
    let sdk = file_system::read_file(PATH)?;

    let parsed_program = parser(sdk)?;

    let validator = validator::Validator::default();
    let (context, program) = validator.validate(parsed_program)?;

    let output_zig = bin_create_dir()?;

    let mut ctx = transpiler::TranspileContext::default();

    let code = ctx.transpile(program);
    file_system::write_file(&output_zig.join("./src/main.zig"), code)?;

    build(output_zig)?;

    Ok(())
}
#[test]
fn compiler_binary_op_test() {
    let sdk = file_system::read_file("../examples/binary_ops.az");
    assert!(sdk.is_ok());

    let parsed_program = parser(sdk.unwrap());
    assert!(parsed_program.is_ok());

    let mut program = parsed_program.unwrap();

    let mut validator = validator::Validator::default();

    let varr = validator.validate(program);
    //
    // let mut ctx = TranspileContext::default();
    // assert_eq!(
    //     ctx.transpile(program),
    //     String::from(
    //         "const std = @import(\"std\"); pub fn main() !void {std.debug.print(\"{}\\n\",.{2 * 2});std.debug.print(\"{}\\n\",.{2 + 2});std.debug.print(\"{}\\n\",.{2 - 2 + 1});std.debug.print(\"{}\\n\",.{2 / 2});std.debug.print(\"{}\\n\",.{2 + 2 / 2});}"
    //     )
    // );
}
#[test]
fn compiler_float_test() {
    let sdk = file_system::read_file("../examples/float.az");
    assert!(sdk.is_ok());

    let parsed_program = parser(sdk.unwrap());
    assert!(parsed_program.is_ok());

    let mut program = parsed_program.unwrap();

    // let mut validator = validator::Validator::new();
    //
    // assert!(validator.validate(&mut program).is_ok());
    //
    // let mut ctx = TranspileContext::default();
    // assert_eq!(
    //     ctx.transpile(program),
    //     String::from(
    //         "const std = @import(\"std\"); pub fn main() !void {var a: f64 = 5.1;std.debug.print(\"{d}\\n\",.{a});a = a + 2 + 1; std.debug.print(\"{d}\\n\",.{a});}"
    //     )
    // );
}
#[test]
fn compiler_print_string_interpolation_test() {
    let sdk = file_system::read_file("../examples/hello_world.az");
    assert!(sdk.is_ok());

    let parsed_program = parser(sdk.unwrap());
    assert!(parsed_program.is_ok());

    let mut program = parsed_program.unwrap();
}
#[test]
fn compiler_condition_test() {
    let sdk = file_system::read_file("../examples/if.az");
    assert!(sdk.is_ok());

    let parsed_program = parser(sdk.unwrap());
    assert!(parsed_program.is_ok());

    let mut program = parsed_program.unwrap();
}
#[test]
fn compiler_function_test() {
    let sdk = file_system::read_file("../examples/square.az");
    assert!(sdk.is_ok());

    let parsed_program = parser(sdk.unwrap());
    assert!(parsed_program.is_ok());

    let mut program = parsed_program.unwrap();
}
// #[test]
// pub fn compiler_array() {
//     let sdk = file_system::read_file("../examples/array.az");
//     assert!(sdk.is_ok());
//
//     let parsed_program = parser(sdk.unwrap());
//     assert!(parsed_program.is_ok());
//
//     let mut program = parsed_program.unwrap();
//
//     let mut validator = validator::Validator::new();
//
//     assert!(validator.validate(&mut program).is_ok());
//
//     //cleaner::clean_ast(&mut program, &validator);
//     let mut ctx = TranspileContext::default();
//     assert_eq!(
//         ctx.transpile(program),
//         String::from(
//             "const std = @import(\"std\"); pub fn main() !void {const b: []const u8 = \"Salam\";std.debug.print(\"{s}\\n\",.{b});}"
//         )
//     );
// }
