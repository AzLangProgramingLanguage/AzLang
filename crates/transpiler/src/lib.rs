use parser::{
    ast::{Expr, FunctionDef, Program},
    shared_ast::BuiltInFunction,
};
use std::collections::{HashMap, HashSet};
pub mod builtin;
use crate::{builtin::print, helper::is_semicolon_needed, transpile::transpile_stmt};
pub mod helper;
mod tests;
pub mod transpile;
pub fn transpile_expr(expr: Expr, ctx: &mut TranspileContext) -> String {
    match expr {
        Expr::String(s) => format!("\"{s}\""),
        Expr::Number(num) => num.to_string(),
        Expr::VariableRef { name, symbol } => {
            if let Some(sym) = symbol
                && sym.is_pointer
            {
                format!("*{name}")
            } else {
                name
            }
        }
        Expr::BuiltInCall {
            function,
            mut args,
            return_type,
        } => match function {
            BuiltInFunction::Print => print::transpile_print(args.swap_remove(0), ctx),

            _ => todo!(),
        },
        _ => String::from("void"),
    }
}

// use std::collections::{HashMap, HashSet};
//
// mod binary_op;
// mod codegen;
// pub mod declaration;
// mod definition;
// mod function_call;
// mod helper;
// pub mod strategy;
// pub mod transpile;
// pub mod variable_ref;
//
// mod zigbuiltin_functions;
// use parser::ast::{Expr, Program};
//
// use crate::{
//     definition::{
//         function_def::transpile_function_def, struct_def::transpile_struct_def,
//         union_def::transpile_union_def,
//     },
//     helper::is_semicolon_needed,
//     transpile::transpile_expr,
// };
//
// type Variable<'a> = HashMap<String, (bool, String)>;
// #[derive(Debug, Clone)]
// struct FunctionDef {
//     is_used_try: bool,
// }
//
#[derive(Clone, Debug, Default)]
pub struct TranspileContext {
    pub imports: HashSet<String>,
    // pub used_try: bool,
    // pub uses_stdout: bool,
    // pub used_min_fn: bool,
    // pub used_max_fn: bool,
    pub functions: HashMap<String, FunctionDef>,
    // pub used_input_fn: bool,
    // pub is_find_method: bool,
    // pub used_sum_fn: bool,
    // pub used_split_n_fn: bool,
    // pub needs_allocator: bool,
    // pub used_split_auto_fn: bool,
    // pub used_split_alloc_fn: bool,
    /*     pub is_used_allocator: bool,
    pub current_struct: Option<&'a str>,
    pub current_union: Option<&'a str>,
    pub uses_stdout: bool,
    pub used_min_fn: bool,
    pub used_max_fn: bool,
    pub enum_defs: HashMap<Cow<'a, str>, Vec<Cow<'a, str>>>,
    pub used_input_fn: bool,
    pub cleanup_statements: Vec<String>,

    pub struct_defs: HashMap<Cow<'a, str>, Cow<'a, Vec<(&'a str, Type<'a>)>>>,

    pub is_used_self: bool, */
}
impl TranspileContext {
    pub fn add_import(&mut self, import: &str) -> Option<String> {
        if self.imports.contains(import) {
            None
        } else {
            self.imports.insert(import.to_string());
            Some(import.to_string())
        }
    }

    pub fn transpile(&mut self, program: Program) -> String {
        let mut body = String::new();

        for stmt in program.expressions {
            if is_semicolon_needed(&stmt) {
                body.push_str(&transpile_stmt(stmt, self));
                body.push(';');
            } else {
                body.push_str(&transpile_stmt(stmt, self));
            }
        }
        let imports = self
            .imports
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(";");
        format!(
            "
        {}
        pub fn main()
        {{
         {body}
        }}    
            ",
            imports
        )
    }
}
//
// impl<'a> TranspileContext<'a> {
//   //     pub fn transpile(&mut self, program: Program) -> String {
//         let mut main_body = String::new();
//         let mut defs = String::new();
//         let mut top_levels = String::new();
//
//         for expr in program.expressions {
//             match expr {
//                 // Expr::FunctionDef {
//                 //     name,
//                 //     params,
//                 //     body,
//                 //     return_type,
//                 // } => {
//                 //     defs.push_str(&transpile_function_def(
//                 //         name,
//                 //         params,
//                 //         body,
//                 //         &return_type,
//                 //         None,
//                 //         self,
//                 //         &false,
//                 //     ));
//                 // }
//                 Expr::StructDef {
//                     name,
//                     fields,
//                     methods,
//                 } => {
//                     // let union = transpile_struct_def(name, fields, methods, self);
//                     // defs.push_str(&union);
//                 }
//                 Expr::UnionType {
//                     name,
//                     fields,
//                     methods,
//                 } => {
//                     let union = transpile_union_def(name, fields, methods, self);
//                     defs.push_str(&union);
//                 }
//                 other => {
//                     let needs_semicolon = is_semicolon_needed(&other);
//                     let transpiled = transpile_expr(other, self);
//                     main_body.push_str(&transpiled);
//                     if needs_semicolon {
//                         main_body.push(';');
//                     }
//                 }
//             }
//         }
//
//         let utils = codegen::utils_fn::generate_util_functions(self);
//         if self.needs_allocator {
//             top_levels.push_str(
//                 "     var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
//         defer arena.deinit();
//
//         const allocator = arena.allocator(); ",
//             );
//         }
//
//         let imports = codegen::prelude::generate_imports(self);
//         return format!(
//             r#"{imports}
//
// {defs}
// {utils}
//
// pub fn main() !void {{
// {top_levels}
// {main_body}}}
// "#,
//         );
//     }
// }
