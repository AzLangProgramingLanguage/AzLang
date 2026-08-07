use crate::{errors::CompilerError, parser};

#[test]
fn compiler_binary_op_test() {
    let sdk = file_system::read_file("../examples/binary_ops.az");
    assert!(sdk.is_ok());

    let parsed_program = parser(sdk.unwrap());
    assert!(parsed_program.is_ok());

    let _ = parsed_program.unwrap();
}
#[test]
fn compiler_float_test() {
    let sdk = file_system::read_file("../examples/float.az");
    assert!(sdk.is_ok());

    let parsed_program = parser(sdk.unwrap());
    assert!(parsed_program.is_ok());

    let _ = parsed_program.unwrap();
}
#[test]
fn compiler_condition_test() {
    let sdk = file_system::read_file("../examples/if.az");
    assert!(sdk.is_ok());

    let parsed_program = parser(sdk.unwrap());
    assert!(parsed_program.is_ok());

    let _ = parsed_program.unwrap();
}
#[test]
fn compiler_function_test() {
    let sdk = file_system::read_file("../examples/square.az");
    assert!(sdk.is_ok());

    let parsed_program = parser(sdk.unwrap());
    assert!(parsed_program.is_ok());

    let _ = parsed_program.unwrap();
}