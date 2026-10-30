#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use parser::shared_ast::Type;
    use validator::ast::{Ast, Expr, Function, Program};

    use crate::Transpiler;

    #[test]
    fn transpile_a_variable() {
        let mut context = Transpiler::default();

        let program = Program {
            variables: HashMap::new(),
            expressions: vec![Ast::Decl {
                name: String::from("a"),
                typ: Type::Natural,
                value: Box::new(validator::ast::Expr::Number(1)),
            }],
            functions: vec![],
        };
        assert_eq!(
            context.transpile_start(program),
            "format ELF64 executable\nsegment readable executable\n\nentry main\nmain:
mov rax, 1
mov rax, 60\nxor rdi, rdi\nsyscall\n"
        );
    }
    #[test]
    fn transpile_asm() {
        let mut context = Transpiler::default();

        let program = Program {
            variables: HashMap::new(),
            expressions: vec![Ast::Asm(Expr::String(String::from("Hello")))],
            functions: vec![],
        };
        assert_eq!(
            context.transpile_start(program),
            "format ELF64 executable\nsegment readable executable\n\nentry main\nmain:
Hello\nmov rax, 60\nxor rdi, rdi\nsyscall\n"
        );
    }
    #[test]
    fn transpile_function() {
        let mut context = Transpiler::default();

        let program = Program {
            variables: HashMap::new(),
            expressions: vec![],
            functions: vec![Function {
                name: String::from("Hello"),
                body: vec![],
                params: vec![],
                return_typ: Type::Void,
            }],
        };
        assert_eq!(
            context.transpile_start(program),
            "format ELF64 executable
segment readable executable\n\nentry main
Hello:
ret
main:
mov rax, 60\nxor rdi, rdi\nsyscall\n"
        );
    }
    #[test]
    fn transpile_function_and_call() {
        let mut context = Transpiler::default();

        let program = Program {
            variables: HashMap::new(),
            expressions: vec![Ast::Expr(Expr::Call {
                target: None,
                name: Box::new(Expr::String("Hello".to_string())),
                args: vec![],
                returned_type: Type::Void,
            })],
            functions: vec![Function {
                name: String::from("Hello"),
                body: vec![],
                params: vec![],
                return_typ: Type::Void,
            }],
        };
        assert_eq!(
            context.transpile_start(program),
            "format ELF64 executable
segment readable executable\n\nentry main
Hello:
ret
main:
call Hello
mov rax, 60\nxor rdi, rdi\nsyscall\n"
        );
    }
}
