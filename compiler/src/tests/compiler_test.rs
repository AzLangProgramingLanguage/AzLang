use crate::parser;
use transpiler::TranspileContext;

#[test]
fn compiler_variable_test() {
    let sdk = file_system::read_file("../examples/variables.az");
    assert!(sdk.is_ok());

    let parsed_program = parser(sdk.unwrap());
    assert!(parsed_program.is_ok());

    let mut program = parsed_program.unwrap();

    let mut validator = validator::Validator::new();

    assert!(validator.validate(&mut program).is_ok());

    let mut ctx = TranspileContext::default();
    assert_eq!(
        ctx.transpile(program),
        String::from(
            "const std = @import(\"std\"); pub fn main() !void {const b: []const u8 = \"Salam\";std.debug.print(\"{s}\\n\",.{b});}"
        )
    );
}

#[test]
fn compiler_binary_op_test() {
    let sdk = file_system::read_file("../examples/binary_ops.az");
    assert!(sdk.is_ok());

    let parsed_program = parser(sdk.unwrap());
    assert!(parsed_program.is_ok());

    let mut program = parsed_program.unwrap();

    let mut validator = validator::Validator::new();

    assert!(validator.validate(&mut program).is_ok());

    let mut ctx = TranspileContext::default();
    assert_eq!(
        ctx.transpile(program),
        String::from(
            "const std = @import(\"std\"); pub fn main() !void {std.debug.print(\"{}\\n\",.{2 * 2});std.debug.print(\"{}\\n\",.{2 + 2});std.debug.print(\"{}\\n\",.{2 - 2 + 1});std.debug.print(\"{}\\n\",.{2 / 2});std.debug.print(\"{}\\n\",.{2 + 2 / 2});}"
        )
    );
}
#[test]
fn compiler_float_test() {
    let sdk = file_system::read_file("../examples/float.az");
    assert!(sdk.is_ok());

    let parsed_program = parser(sdk.unwrap());
    assert!(parsed_program.is_ok());

    let mut program = parsed_program.unwrap();

    let mut validator = validator::Validator::new();

    assert!(validator.validate(&mut program).is_ok());

    let mut ctx = TranspileContext::default();
    assert_eq!(
        ctx.transpile(program),
        String::from(
            "const std = @import(\"std\"); pub fn main() !void {var a: f64 = 5.1;std.debug.print(\"{d}\\n\",.{a});a = a + 2 + 1; std.debug.print(\"{d}\\n\",.{a});}"
        )
    );
}
#[test]
fn compiler_print_string_interpolation_test() {
    let sdk = file_system::read_file("../examples/hello_world.az");
    assert!(sdk.is_ok());

    let parsed_program = parser(sdk.unwrap());
    assert!(parsed_program.is_ok());

    let mut program = parsed_program.unwrap();

    let mut validator = validator::Validator::new();

    assert!(validator.validate(&mut program).is_ok());

    let mut ctx = TranspileContext::default();
    assert_eq!(
        ctx.transpile(program),
        String::from(
            "const std = @import(\"std\"); pub fn main() !void {std.debug.print(\"Salam {} \\n\",.{1312});}"
        )
    );
}
#[test]
fn compiler_condition_test() {
    let sdk = file_system::read_file("../examples/if.az");
    assert!(sdk.is_ok());

    let parsed_program = parser(sdk.unwrap());
    assert!(parsed_program.is_ok());

    let mut program = parsed_program.unwrap();

    let mut validator = validator::Validator::new();

    assert!(validator.validate(&mut program).is_ok());

    let mut ctx = TranspileContext::default();
    assert_eq!(
        ctx.transpile(program),
        String::from(
            "const std = @import(\"std\"); pub fn main() !void {if(1 != 1\n) { std.debug.print(\"Salam\\n\",.{}); }      \n      else if (2 != 3) { std.debug.print(\"2\\n\",.{}); } else { std.debug.print(\"3\\n\",.{}); } }"
        )
    );
}
#[test]
fn compiler_function_test() {
    let sdk = file_system::read_file("../examples/square.az");
    assert!(sdk.is_ok());

    let parsed_program = parser(sdk.unwrap());
    assert!(parsed_program.is_ok());

    let mut program = parsed_program.unwrap();

    let mut validator = validator::Validator::new();

    assert!(validator.validate(&mut program).is_ok());

    let mut ctx = TranspileContext::default();
    assert_eq!(
        ctx.transpile(program),
        String::from(
            "const std = @import(\"std\"); pub fn main() !void {if(1 != 1\n) { std.debug.print(\"Salam\\n\",.{}); }      \n      else if (2 != 3) { std.debug.print(\"2\\n\",.{}); } else { std.debug.print(\"3\\n\",.{}); } }"
        )
    );
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
