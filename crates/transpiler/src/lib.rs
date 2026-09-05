use validator::ast::{
    Ast::{self},
    Expr, Program,
};
#[derive(Default)]
pub struct Transpiler {
    data: Vec<String>,
    stack: Vec<String>,
}
impl Transpiler {
    fn push_temp_stack(&mut self, expr: Expr) -> String {
        match expr {
            Expr::Number(num) => {
                self.stack
                    .push(format!("%num{} =w add 0,{num}", self.stack.len()));
                format!("w %num{}", self.stack.len() - 1)
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
                self.stack.push(format!(
                    "%bin{} = w add {},{}",
                    self.stack.len(),
                    *left,
                    *right
                ));
                format!("w %bin{}", self.stack.len() - 1)
            }

            _ => todo!("there is not complated yet. acutally, i dont know what to do "),
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
        } else {
            self.push_temp_stack(expr);
        }
    }

    pub fn transpile(&mut self, program: Program) -> String {
        let mut exprstream = String::new();
        for ast in program.expressions {
            match ast {
                Ast::Expr(expr) => self.expr_transpiler(&mut exprstream, expr),
                _ => {}
            }
        }
        format!(
            "{}
  export function w $main() {{
             @start
                 {}
         {exprstream}
                 ret
             }}
",
            self.data.join(""),
            self.stack.join("")
        )
    }
}
