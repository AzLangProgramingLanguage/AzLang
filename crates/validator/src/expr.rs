use parser::{
    ast::Symbol,
    shared_ast::{BuiltInFunction, StringEnum, Type},
};

use crate::{
    Validator,
    ast::{self, Ast},
    errors::ValidatorError,
    helper::get_type,
};
type ParserExpr = parser::ast::Expr;
type ValidatorExpr = ast::Expr;

pub fn validate_expr(
    expr: ParserExpr,
    ctx: &mut Validator,
) -> Result<ValidatorExpr, ValidatorError> {
    match expr {
        ParserExpr::String(s) => Ok(ValidatorExpr::String(s)),
        ParserExpr::Number(n) => Ok(ValidatorExpr::Number(n)),
        ParserExpr::VariableRef { name, .. } => {
            let symbol = ctx.lookup_variable_mut_with_err(&name)?;
            symbol.is_used = true;
            Ok(ValidatorExpr::VariableRef(name))
        }
        ParserExpr::BuiltInCall { function, mut args } => {
            let mut validated_args = Vec::new();
            while let Some(arg) = args.pop() {
                validated_args.push(validate_expr(arg, ctx)?);
            }

            let return_type = match &function {
                &BuiltInFunction::Print => Type::Void,
                &BuiltInFunction::Input => Type::String(StringEnum::DynamicString),
                &BuiltInFunction::Len => Type::Natural,
                &BuiltInFunction::Number => Type::Integer,
                &BuiltInFunction::Sum => Type::Integer,
                &BuiltInFunction::Range => Type::Array(Box::new(Type::Integer)),
                &BuiltInFunction::LastWord => Type::Void,
                &BuiltInFunction::Timer => Type::Integer,
                &BuiltInFunction::Max => Type::Integer,
                &BuiltInFunction::Zig => Type::Void,
                &BuiltInFunction::StrLower
                | &BuiltInFunction::StrUpper
                | &BuiltInFunction::Trim
                | &BuiltInFunction::StrReverse
                | &BuiltInFunction::ConvertString => Type::String(StringEnum::DynamicString),
                &BuiltInFunction::Allocator => Type::Void,
                &BuiltInFunction::Min => Type::Integer,
                &BuiltInFunction::Sqrt => Type::Float,
                &BuiltInFunction::Mod => Type::Integer,
                &BuiltInFunction::Ceil => Type::Integer,
                &BuiltInFunction::Floor => Type::Integer,
                &BuiltInFunction::Round => Type::Integer,
            };

            Ok(ValidatorExpr::BuiltInCall {
                function,
                args: validated_args,
                return_type,
            })
        }
        _ => todo!(), // Expr::String(_) | Expr::Float(_) | Expr::Bool(_) | Expr::Number(_) => Ok(()),
                      // Expr::Call {
                      //     target,
                      //     name,
                      //     args,
                      //     returned_type,
                      // } => Ok(()),
                      // Expr::VariableRef { name, symbol } => {
                      //     if let Some(var) = ctx.lookup_variable(name) {
                      //         // var.is_used = true;
                      //         // *symbol = Some(var.clone()); //TODO: Clone etmək əvəzinə reference istifadə etməliyik
                      //         Ok(())
                      //     } else {
                      //         return Err(ValidatorError::UndefinedVariable(name.to_string()));
                      //     }
                      // }
                      // Expr::List(ex) => {
                      //     let mut iter = ex.iter();
                      //
                      //     let typ = match iter.next() {
                      //         Some(first) => get_type(first, ctx),
                      //         None => Type::Any,
                      //     };
                      //
                      //     for e in iter {
                      //         let current_type = get_type(e, ctx);
                      //
                      //         if current_type != typ {
                      //             return Err(ValidatorError::TypeMismatch {
                      //                 expected: typ,
                      //                 found: current_type,
                      //             });
                      //         }
                      //     }
                      //
                      //     Ok(())
                      // }
                      // Expr::BinaryOp {
                      //     left,
                      //     right,
                      //     op,
                      //     return_type,
                      // } => {
                      //     validate_expr(left, ctx)?;
                      //     validate_expr(right, ctx)?;
                      //     let returned_type = get_type(
                      //         &Expr::BinaryOp {
                      //             left: left.clone(),
                      //             right: right.clone(),
                      //             op: *op,
                      //             return_type: Type::Any,
                      //         },
                      //         ctx,
                      //     );
                      //     *return_type = returned_type;
                      //     Ok(())
                      // }
                      // Expr::Index {
                      //     target,
                      //     index,
                      //     target_type,
                      // } => Ok(()),
                      // Expr::Return(e) => {
                      //     validate_expr(&mut **e, ctx)?;
                      //     Ok(())
                      // }
                      // Expr::TemplateString(chunk) => {
                      //     for ch in chunk {
                      //         if let TemplateChunk::Expr(expr) = ch {
                      //             validate_expr(expr, ctx)?;
                      //         }
                      //     }
                      //     Ok(())
                      // }
                      // other => {
                      //     println!("Bura baxmaq lazımdır {:#?}", other);
                      //     todo!("Bura baxmaq lazımdır")
                      // }
    }
}
