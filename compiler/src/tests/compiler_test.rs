#[cfg(test)]
mod __tests__ {
    use crate::cleaner;
    use crate::parser;
    use transpiler::TranspileContext;

    #[test]
    pub fn compiler_test() {
        let sdk = file_system::read_file("../examples/variables.az");
        assert!(sdk.is_ok());

        let parsed_program = parser(sdk.unwrap());
        assert!(parsed_program.is_ok());

        let mut program = parsed_program.unwrap();

        let mut validator = validator::Validator::new();

        assert!(validator.validate(&mut program).is_ok());

        //cleaner::clean_ast(&mut program, &validator);
        let mut ctx = TranspileContext::default();
        assert_eq!(ctx.transpile(program), String::from(""));
    }
}
