use crate::runner::{Runner, Variable, eval::eval, runner::runner_interpretator};
use parser::{ast::Expr, shared_ast::Type};
// pub fn handle_string_call<'a>(
//     name: &str,
//     s: &'a str,
//     args: Vec<Expr<'a>>,
//     ctx: &mut Runner<'a>,
// ) -> Option<Expr<'a>> {
//     let method_body = {
//         let uniontype = ctx.uniontypes.get("Yazı")?;
//         let method = uniontype.methods.iter().find(|m| m.name == name)?;
//         method.body.clone()
//     };
//
//     ctx.variables.insert(
//         "self".to_string(),
//         Variable {
//             value: Expr::String(s),
//             typ: Type::User("Yazı".into()),
//             is_mutable: false,
//         },
//     );
//
//     for expr in method_body {
//         match expr {
//             Expr::Return(value) => {
//                 let val = eval(&*value, ctx);
//                 ctx.variables.remove("self");
//                 return Some(val);
//             }
//             Expr::Comment(c) if c.trim() == "Burasını Sistem Qərar Versin" => match name {
//                 "tərs" => {
//                     let strr = s.chars().rev().collect::<String>();
//                     return Some(Expr::DynamicString(strr.into()));
//                 }
//                 "böyüt" => {
//                     let strr = s.to_uppercase();
//                     return Some(Expr::DynamicString(strr.into()));
//                 }
//                 "kiçilt" => {
//                     let strr = s.to_lowercase();
//                     return Some(Expr::DynamicString(strr.into()));
//                 }
//                 "qırx" => {
//                     let strr = s.trim();
//                     return Some(Expr::String(strr.into()));
//                 }
//                 "uzunluq" => {
//                     let strr = s.len();
//                     return Some(Expr::Number(strr.try_into().unwrap()));
//                 }
//                 "tipi" => {
//                     return Some(Expr::String("Yazı".into()));
//                 }
//                 "boşdur" => {
//                     let strr = s.is_empty();
//                     return Some(Expr::Bool(strr));
//                 }
//                 "böl" => {
//                     let arg = args.get(0).unwrap();
//                     if let Expr::String(arg) = arg {
//                         let strr = s
//                             .split(arg)
//                             .filter(|s| !s.is_empty())
//                             .map(|s| Expr::String(s.into()))
//                             .collect::<Vec<Expr<'a>>>();
//                         return Some(Expr::List(strr));
//                     }
//                 }
//                 "əvəzlə" => {
//                     let arg = args.get(0).unwrap();
//                     if let Expr::String(arg) = arg {
//                         let strr = s.replace(arg, "");
//                         return Some(Expr::DynamicString(strr.into()));
//                     }
//                 }
//                 "başlayırmı" => {
//                     let arg = args.get(0).unwrap();
//                     if let Expr::String(arg) = arg {
//                         let strr = s.starts_with(arg);
//                         return Some(Expr::Bool(strr));
//                     }
//                 }
//                 "bitirirmi" => {
//                     let arg = args.get(0).unwrap();
//                     if let Expr::String(arg) = arg {
//                         let strr = s.ends_with(arg);
//                         return Some(Expr::Bool(strr));
//                     }
//                 }
//                 _ => {}
//             },
//             _ => {
//                 if let Some(val) = runner_interpretator(ctx, expr.clone()) {
//                     return Some(val);
//                 }
//             }
//         }
//     }
//
//     ctx.variables.remove("self");
//     Some(Expr::Void)
// }
