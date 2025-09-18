use crate::interpretator::runner_interpretator::eval;
use crate::parser::ast::{TemplateChunk, Type};
use crate::transpiler::helpers::get_expr_type;
use crate::{interpretator::InterPretator, parser::ast::Expr};

pub fn print_interpreter(expr: &Expr, ctx: &InterPretator) {
    match expr {
        Expr::TemplateString(chunks) => {
            for chunk in chunks {
                match chunk {
                    TemplateChunk::Literal(s) => print!("{}", s),
                    TemplateChunk::Expr(expr) => print!("{}", exporter(&*expr, ctx)),
                }
            }
            print!("\n");
        }
        _ => {
            let arg = eval(expr, ctx);
            println!("{}", exporter(&arg, ctx));
        }
    }
}

pub fn exporter(expr: &Expr, ctx: &InterPretator) -> String {
    match expr {
        Expr::String(s, _) => s.to_string(),
        Expr::Number(n) => n.to_string(),
        Expr::Float(f) => f.to_string(),
        Expr::DynamicString(s) => s.to_string(),
        Expr::Bool(b) => b.to_string(),
        Expr::Char(c) => c.to_string(),
        Expr::VariableRef { name, .. } => {
            if let Some(var) = ctx.variables.get(&name.to_string()) {
                exporter(&var.value, ctx)
            } else {
                format!("<undef:{}>", name)
            }
        }
        Expr::BinaryOp { left, op, right } => {
            format!("({} {} {})", exporter(left, ctx), op, exporter(right, ctx))
        }
        Expr::BuiltInCall { function, args, .. } => {
            let arg_strs: Vec<String> = args.iter().map(|a| exporter(a, ctx)).collect();
            format!("{}({})", function, arg_strs.join(", "))
        }
        Expr::List(list) => {
            let elems: Vec<String> = list.iter().map(|e| exporter(e, ctx)).collect();
            format!("[{}]", elems.join(", "))
        }
        _ => "<unknown>".to_string(),
    }
}
