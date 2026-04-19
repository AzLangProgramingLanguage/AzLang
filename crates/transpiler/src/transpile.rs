use parser::ast::Statement;

use crate::{
    TranspileContext,
    helper::{map_typ, transpile_body},
    transpile_expr,
};

// use crate::{
//     TranspileContext,
//     binary_op::transpile_binary_op,
//     builtin::{
//         input::transpile_input,
//         len::transpile_len,
//         min_max::{transpile_max, transpile_min},
//         print::transpile_print,
//         sum::transpile_sum,
//     },
//     function_call::transpile_function_call,
//     helper::{get_expr_type, get_format_str_from_type, map_type, transpile_body},
//     strategy::VariableDecl,
// };

pub fn transpile_stmt(stmt: Statement, ctx: &mut TranspileContext) -> String {
    match stmt {
        Statement::Decl {
            name,
            typ,
            is_mutable,
            value,
        } => {
            let typ_str = map_typ(&typ);
            let is_mutable_str = if is_mutable { "var" } else { "const" };

            format!(
                "{is_mutable_str} {name}: {typ_str} = {}",
                transpile_expr(*value, ctx)
            )
        }
        Statement::Condition { main, elif, other } => {
            let mut result = String::from("if(");
            result.push_str(&format!(
                "{}
) {{ {} }}      
      ",
                transpile_expr(*main.condition, ctx),
                transpile_body(main.body, ctx)
            ));

            for el in elif {
                result.push_str(&format!(
                    "else if ({}) {{ {} }} ",
                    transpile_expr(*el.condition, ctx),
                    transpile_body(el.body, ctx)
                ));
            }

            if let Some(_else_) = other {
                result.push_str(&format!("else {{ {} }} ", transpile_body(_else_.body, ctx)));
            }
            result
        }
        Statement::Expr(expr) => transpile_expr(expr, ctx),
        other => String::from(""),
    }
}
//
// pub fn transpile_expr<'a>(expr: Expr, ctx: &mut TranspileContext<'a>) -> String {
//     match expr {
//         Expr::String(s) => format!("\"{}\"", s.escape_default()),
//         Expr::DynamicString(s) => format!("try allocator.dupe(u8, \"{}\")", s.escape_default()),
//         Expr::TemplateString(s) => {
//             let mut literal_str = String::new();
//             let mut args = String::from(",.{");
//             for pat in s {
//                 match pat {
//                     TemplateChunk::Literal(s) => {
//                         literal_str.push_str(&s);
//                     }
//                     TemplateChunk::Expr(e) => {
//                         literal_str.push_str(get_format_str_from_type(&get_expr_type(&e), false));
//                         args.push_str(&format!("{},", transpile_expr(*e, ctx)));
//                     }
//                 }
//             }
//             args.pop();
//             args.push('}');
//             format!("try std.fmt.allocPrint(allocator, comptime \"{literal_str}\"{args})",)
//         }
//         Expr::Number(n) => n.to_string(),
//         Expr::Float(n) => {
//             let s = n.to_string();
//             if s.contains('.') || s.contains('e') {
//                 s
//             } else {
//                 format!("{}.0", s)
//             }
//         }
//         Expr::Bool(n) => n.to_string(),
//         Expr::Break => "break".to_string(),
//         Expr::Continue => "continue".to_string(),
//         Expr::VariableRef {
//             name,
//             symbol: Some(Symbol {
//                 is_pointer: true, ..
//             }),
//         } => name.to_string(),
//         Expr::Call {
//             target,
//             name,
//             args,
//             returned_type,
//         } => transpile_function_call(ctx, target, name, args, returned_type),
//         Expr::Loop {
//             var_name,
//             iterable,
//             body,
//         } => {
//             let iterable_str = transpile_expr(*iterable, ctx);
//             let body_str = transpile_body(body, ctx);
//             format!("for ({iterable_str})  |{var_name}| {{ {body_str} }}")
//         }
//         Expr::BinaryOp {
//             left,
//             right,
//             op,
//             return_type,
//         } => transpile_binary_op(ctx, left, right, op, return_type),
//         Expr::Decl {
//             name,
//             typ,
//             is_mutable,
//             value,
//         } => VariableDecl::transpile(name, &*typ, is_mutable, *value, ctx),
//         Expr::Condition { main, elif, other } => {
//             let mut condition_str = format!(
//                 "if ({}) {{ {} }}",
//                 transpile_expr(*main.condition, ctx),
//                 transpile_body(main.body, ctx)
//             );
//
//             for branch in elif {
//                 condition_str.push_str(&format!(
//                     " else if ({}) {{ {} }}",
//                     transpile_expr(*branch.condition, ctx),
//                     transpile_body(branch.body, ctx)
//                 ));
//             }
//
//             if let Some(other) = other {
//                 condition_str.push_str(&format!(" else {{ {} }}", transpile_body(other.body, ctx)));
//             }
//
//             condition_str
//         }
//
//         Expr::BuiltInCall {
//             function, mut args, ..
//         } => match function {
//             BuiltInFunction::Print => transpile_print(args.remove(0), ctx),
//             BuiltInFunction::Sum => transpile_sum(&mut args, ctx),
//             BuiltInFunction::Input => transpile_input(args.remove(0), ctx),
//
//             BuiltInFunction::Min => transpile_min(&mut args, ctx),
//             BuiltInFunction::Max => transpile_max(&mut args, ctx),
//             BuiltInFunction::Len => transpile_len(&mut args, ctx),
//
//             _ => "None".to_string(),
//         },
//         Expr::List(list) => {
//             let mut str_list = String::new();
//             let str_type = if list.get(0).is_some() {
//                 map_type(&get_expr_type(&list[0]), true)
//             } else {
//                 "isize"
//             };
//             for expr in list {
//                 str_list.push_str(&transpile_expr(expr, ctx));
//                 str_list.push(',');
//             }
//             str_list.pop();
//             format!("[_]{str_type}{{{str_list}}}")
//         }
//         Expr::Assignment {
//             name,
//             value,
//             symbol: _,
//         } => {
//             let transpiled = transpile_expr(*value, ctx);
//             format!("{name} = {transpiled} ")
//         }
//         Expr::Return(value) => {
//             let transpiled_value = transpile_expr(*value, ctx);
//
//             format!("return {transpiled_value}")
//         }
//         Expr::VariableRef { name, symbol } => {
//             if let Some(sym) = symbol
//                 && sym.is_pointer
//             {
//                 return format!("*{name}");
//             }
//             name
//         }
//         other => {
//             println!("{:?}", other);
//             panic!("Error")
//         }
//     }
// }
