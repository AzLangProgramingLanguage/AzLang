use parser::{self, shared_ast::Type};
use std::{collections::HashMap, io::Write};
use validator::ast::{
    Ast::{self},
    Expr, Program,
};
#[derive(Default)]
pub struct Transpiler {
    variables: HashMap<String, Expr>,
    data: Vec<String>,
    stack: Vec<String>,
    labels: String,
    conditions: u64,
}
impl Transpiler {
    fn transpile_program(&mut self) {}
    fn string(&mut self, var_name: Option<String>, val: String) {
        let variable_name = match var_name {
            Some(str) => str,
            None => String::from("azlang_data"),
        };
        let datalen = self.data.len();
        self.data.push(format!("{variable_name}{datalen} db '{val}',0\n{variable_name}{datalen}_len = $ - {variable_name}{datalen}"));
    }
    pub fn transpile_start(&mut self, program: Program) -> String {
        self.string(Some(String::from("message")), String::from("Hello World"));
        format!(
            "
                
format ELF64 executable

segment readable executable
entry main
main:
    mov rdi,1
    lea rsi, [message0]
    mov rdx, message0_len
    call write
    mov rax, 60
    xor rdi, rdi
    syscall
{}           ",
            self.data.join("")
        )
    }
    pub fn transpile_module(&mut self, module: &str, program: Program) -> String {
        String::new()
    }
}
