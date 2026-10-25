use parser::{self, shared_ast::Type};
use std::{collections::HashMap, io::Write};
use validator::ast::{
    Ast::{self},
    Expr, Function, Program,
};
#[derive(Default)]
pub struct Transpiler {
    variables: HashMap<String, Expr>,
    data: Vec<String>,
    stack: Vec<String>,
    labels: String,
    conditions: u64,
    includes: String,
}
impl Transpiler {
    fn transpile_program(&mut self, expressions: Vec<Ast>) -> String {
        let mut output = String::new();
        for ast in expressions {
            output.push_str(&self.transpile_ast(ast));
        }
        output
    }
    fn string(&mut self, var_name: Option<String>, val: String) {
        let variable_name = match var_name {
            Some(str) => str,
            None => String::from("azlang_data"),
        };
        let datalen = self.data.len();
        self.data.push(format!("{variable_name}{datalen} db '{val}',0\n{variable_name}{datalen}_len = $ - {variable_name}{datalen}"));
    }
    fn transpile_ast(&mut self, ast: Ast) -> String {
        match ast {
            Ast::Asm(expr) => {
                let mut asm = self.transpile_expr(expr);
                asm.push('\n');
                return asm;
            }

            Ast::Decl { name, typ, value } => {
                format!("mov rax, {value}\n")
            }
            _ => todo!(),
        }
    }
    fn transpile_expr(&mut self, expr: Expr) -> String {
        match expr {
            Expr::String(s) => s.replace("\\n", "\n"),
            _ => todo!(),
        }
    }
    fn transpile_labels(&mut self, functions: Vec<Function>) {
        for func in functions {
            self.labels.push_str(&format!("{}:\n", func.name));
            if func.body.is_empty() {
                self.labels.push_str(&String::from("ret\n"));
            }
            for b in func.body {
                let transpiled = self.transpile_ast(b);
                self.labels.push_str(&transpiled);
            }
        }
    }

    pub fn transpile_start(&mut self, program: Program) -> String {
        let output = self.transpile_program(program.expressions);
        self.transpile_labels(program.functions);
        format!(
            "format ELF64 executable
segment readable executable
{}
entry main
{}main:
{output}mov rax, 60
xor rdi, rdi
syscall
{}",
            self.data.join(""),
            self.labels,
            self.includes
        )
    }
    pub fn transpile_module(&mut self, module: &str, program: Program) -> String {
        String::new()
    }
}
mod tests;
