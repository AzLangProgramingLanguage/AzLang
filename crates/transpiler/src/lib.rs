use parser::{self, shared_ast::Type};
use std::{collections::HashMap, io::Write};
use validator::ast::{
    Ast::{self},
    Expr, Function, Program,
};
#[derive(Default)]
pub struct Transpiler {
    variables: Vec<HashMap<String, Expr>>,
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
            match ast {
                Ast::Asm(expr) => {
                    let mut asm = self.transpile_expr(expr);
                    asm.push('\n');
                    output.push_str(&asm);
                }
                Ast::Decl { name, typ, value } => {
                    self.decl(name, *value);
                }
                Ast::Expr(expr) => {
                    output.push_str(&self.transpile_expr(expr));
                }
                _ => todo!(),
            }
        }
        output
    }

    fn decl(&mut self, name: String, value: Expr) {
        if let Some(stack) = self.variables.last_mut() {
            stack.insert(name, value);
        } else {
            let mut hashmap: HashMap<String, Expr> = HashMap::new();
            hashmap.insert(name, value);
            self.variables.push(hashmap);
        }
    }
    fn find_variable(&self, name: &String) -> &Expr {
        for var in self.variables.iter().rev() {
            if let Some(var) = var.get(name) {
                return var;
            }
        }
        panic!("There is no any variable")
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
            _ => todo!(),
        }
    }
    fn transpile_expr(&mut self, expr: Expr) -> String {
        match expr {
            Expr::String(s) => s.replace("\\n", "\n"), //TODO: You don't Need this
            Expr::VariableRef { name, symbol } => {
                self.transpile_expr(self.find_variable(&name).clone())
            }
            Expr::Call {
                target,
                name,
                args,
                returned_type,
            } => {
                format!("call {}\n", self.transpile_expr(*name))
            }
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
