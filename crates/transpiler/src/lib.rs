use parser::{self, shared_ast::Type};
use std::io::Write;
use validator::ast::{
    Ast::{self},
    Expr, Program,
};
#[derive(Default)]
pub struct Transpiler {
    data: Vec<String>,
    stack: Vec<String>,
    labels: String,
    conditions: u64,
}
impl Transpiler {
    fn push_temp_stack(&mut self, expr: Expr) -> String {
        match expr {
            Expr::VariableRef { name, symbol } => {
                match symbol.typ {
                    Type::String(_) => {
                        return format!("l ${name}");
                    }
                    Type::Integer => {
                        return format!("w %{name}");
                    }
                    Type::User(_) => {
                        return format!("w %{name}");
                    }
                    other => todo!("{other} is not implemented yet"),
                }
                format!("w %{name}")
            }
            Expr::Number(num) => {
                format!("l {num}")
            }
            Expr::String(str) => {
                self.data.push(format!(
                    "data $str{} ={{ b \"{str}\", b 0 }} ",
                    self.data.len()
                ));
                format!("l $str{}", self.data.len() - 1)
            }
            Expr::BinaryOp {
                left,
                right,
                op,
                return_type,
            } => {
                match op {
                    parser::ast::Operation::Add => self.stack.push(format!(
                        "%bin{} = w add {},{}",
                        self.stack.len(),
                        *left,
                        *right
                    )),
                    parser::ast::Operation::Greater => self.stack.push(format!(
                        "%bin{} = w cgt {},{}",
                        self.stack.len(),
                        *left,
                        *right
                    )),
                    parser::ast::Operation::Not => self.stack.push(format!(
                        "%bin{} = w ceqw {},{}",
                        self.stack.len(),
                        *left,
                        *right //Bug
                    )),

                    parser::ast::Operation::Subtract => self.stack.push(format!(
                        "%bin{} = w sub {},{}",
                        self.stack.len(),
                        *left,
                        *right
                    )),
                    parser::ast::Operation::Multiply => self.stack.push(format!(
                        "%bin{} = w mul {},{}",
                        self.stack.len(),
                        *left,
                        *right
                    )),
                    parser::ast::Operation::Divide => self.stack.push(format!(
                        "%bin{} = w div {},{}",
                        self.stack.len(),
                        *left,
                        *right
                    )),

                    parser::ast::Operation::Equal => self.stack.push(format!(
                        "%bin{} = w ceqd {},{}",
                        self.stack.len(),
                        *left,
                        *right
                    )),

                    parser::ast::Operation::NotEqual => self.stack.push(format!(
                        "%bin{} = w cnew {},{}",
                        self.stack.len(),
                        *left,
                        *right
                    )),
                    parser::ast::Operation::GreaterEqual => self.stack.push(format!(
                        "%bin{} = w csgew {},{}",
                        self.stack.len(),
                        *left,
                        *right
                    )),
                    other => todo!("This is not implemented yet {other:?}"),
                };
                format!("%bin{}", self.stack.len() - 1)
            }
            Expr::Void => String::from(""),

            other => {
                todo!("{other:?} there is not complated yet. acutally, i dont know what to do ")
            }
        }
    }

    fn expr_transpiler(&mut self, stream: &mut String, expr: Expr) {
        if let Expr::Call {
            target, args, name, ..
        } = expr
        {
            stream.push_str(&format!("call ${name}("));
            let argslen = args.len();

            for (index, arg) in args.into_iter().enumerate() {
                let variable = self.push_temp_stack(arg);
                stream.push_str(&variable);
                if index < argslen - 1 {
                    stream.push(',');
                }
            }
            stream.push(')');
            stream.push('\n');
        } else {
            stream.push_str(&self.push_temp_stack(expr));
        }
    }
    pub fn transpile_body(&mut self, exprstream: &mut String, body: Vec<Ast>) {
        for ast in body {
            match ast {
                Ast::Expr(expr) => self.expr_transpiler(exprstream, expr),
                Ast::Condition { main, elif, other } => {
                    self.conditions += 1;
                    let condition_num = self.conditions;
                    let mut condition = String::new();
                    self.expr_transpiler(&mut condition, *main.condition);
                    exprstream.push_str(&format!(
                        "jnz {condition}, @true{condition_num}, @false{condition_num}\n"
                    ));
                    exprstream.push_str(&format!("@true{condition_num}\n"));
                    self.transpile_body(exprstream, main.body);
                    exprstream.push_str(&format!(
                        "jmp @continue{condition_num}\n@false{condition_num}\n"
                    ));
                    if let Some(els) = other {
                        self.transpile_body(exprstream, els.body);
                    }
                    exprstream.push_str(&format!(
                        "jmp @continue{condition_num}\n@continue{condition_num}\n"
                    ));
                }
                Ast::Loop { body } => {
                    exprstream.push_str(&format!("@loop1\n"));
                    exprstream.push_str(&String::from("jmp @loop1\n"));
                    exprstream.push_str(&String::from("@continue2\n"));
                }
                Ast::Decl {
                    name,
                    typ,
                    is_mutable,
                    value,
                } => match typ {
                    Type::Integer => {
                        self.stack.push(format!("%{name} = w copy {value}\n"));
                    }
                    Type::Natural => {
                        self.stack.push(format!("%{name} = w copy {value}\n"));
                    }

                    Type::String(strenum) => {
                        self.data
                            .push(format!("data ${name} = {{ b {value}, b 0 }}\n"));
                    }
                    other => todo!("{other:?} Not implemented yet"),
                },
                _ => {}
            }
        }
    }

    pub fn transpile(&mut self, program: Program) -> String {
        let mut exprstream = String::new();
        self.transpile_body(&mut exprstream, program.expressions);
        format!(
            "{}
export function w $_start() {{
@start
{}
{exprstream}
call $exit(w 0)
ret
}}
",
            self.data.join("\n"),
            self.stack.join(""),
        )
    }
}
