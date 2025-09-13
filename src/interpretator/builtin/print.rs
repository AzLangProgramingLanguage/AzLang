use crate::parser::ast::Type;
use crate::transpiler::helpers::get_expr_type;
use crate::{interpretator::InterPretator, parser::ast::Expr};

pub fn print_interpreter(expr: &Expr, ctx: &InterPretator) {
    println!("{}", exporter(expr, ctx));
}

pub fn exporter(expr: &Expr, ctx: &InterPretator) -> String {
    match expr {
        Expr::String(s, _) => s.to_string(),
        Expr::VariableRef {
            name,
            transpiled_name: _,
            symbol: _,
        } => {
            let variable = ctx.variables.get(&name.to_string());
            if let Some(variable) = variable {
                return exporter(&variable.value, ctx);
            }
            String::new()
        }
        Expr::Number(n) => n.to_string(),
        Expr::BinaryOp { left, op, right } => {
            let left_type = get_expr_type(left);
            if let Type::Integer | Type::Natural | Type::Float = left_type {
                let result = match *op {
                    "+" => {
                        let left_number = match &**left {
                            /*                             Expr::Number(n) => n,
                             */
                            Expr::VariableRef { name, .. } => {
                                let variable = ctx.variables.get(&name.to_string());
                                if let Some(variable) = variable {
                                    return exporter(&variable.value, ctx);
                                }
                                0
                            }
                            _ => 0,
                        };
                        let right_number = match &**right {
                            /*                             Expr::Number(n) => n,
                             */
                            Expr::VariableRef { name, .. } => {
                                let variable = ctx.variables.get(&name.to_string());
                                if let Some(variable) = variable {
                                    return exporter(&variable.value, ctx);
                                }
                                0
                            }
                            _ => 0,
                        };
                        left_number + right_number
                    }
                    _ => 0,
                };
            }
            "".to_string()
        }
        Expr::Float(f) => f.to_string(),
        Expr::Bool(b) => b.to_string(),
        Expr::BuiltInCall {
            function,
            args,
            return_type,
        } => {
            let mut s = String::new();
            s.push_str(&function.to_string());
            s.push_str("(");
            for (i, arg) in args.iter().enumerate() {
                s.push_str(&exporter(arg, ctx));
                if i < args.len() - 1 {
                    s.push_str(", ");
                }
            }
            s.push(')');
            s
        }
        Expr::Char(c) => c.to_string(),
        Expr::List(l) => {
            let mut s = String::new();
            s.push('[');
            for (i, expr) in l.iter().enumerate() {
                s.push_str(&exporter(expr, ctx));
                if i < l.len() - 1 {
                    s.push_str(", ");
                }
            }
            s.push(']');
            s
        }
        _ => String::new(),
    }
}
