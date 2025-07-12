use super::{Parser, Token};
use crate::{
    ValidatorContext,
    parser::{
        Expr,
        ast::{BuiltInFunction, Type},
    },
}; // `Type` enumunu import et

pub fn parse_type(parser: &mut Parser) -> Result<Type, String> {
    let base_token = parser.next();
    let base = match base_token {
        Some(Token::TypeName(t)) => t.clone(),
        Some(Token::Identifier(ident)) => Type::Istifadeci(ident.clone()),
        Some(Token::Integer) => Type::Integer,
        Some(Token::BigInteger) => Type::BigInteger,
        Some(Token::LowInteger) => Type::LowInteger,
        Some(Token::String) => Type::Metn,
        Some(Token::Array) => {
            match parser.next() {
                Some(Token::Operator(op)) if op == "<" => (),
                other => return Err(format!("'<' gözlənilirdi, tapıldı: {:?}", other)),
            }

            let inner_type = parse_type(parser)?;

            match parser.next() {
                Some(Token::Operator(op)) if op == ">" => (),
                other => return Err(format!("'>' gözlənilirdi, tapıldı: {:?}", other)),
            }

            Type::Siyahi(Box::new(inner_type))
        }
        other => return Err(format!("Tip gözlənilirdi, tapıldı: {:?}", other)),
    };

    Ok(base)
}

pub fn get_type(expr: &Expr, ctx: &ValidatorContext) -> Option<Type> {
    match expr {
        Expr::Index { target, .. } => {
            let target_type = get_type(target, ctx)?;

            match target_type {
                Type::Siyahi(inner) => Some(*inner),
                Type::Metn => Some(Type::Char),
                _ => None,
            }
        }
        Expr::Float(_) => Some(Type::Float),
        Expr::Match(match_expr) => {
            // target tipi yoxlaya bilərsən, lazım olsa
            let _target_type = get_type(&match_expr.target, ctx)?;

            let mut arm_types = Vec::new();

            for (_variant, exprs) in &match_expr.arms {
                if exprs.is_empty() {
                    return None;
                }

                let last_expr = exprs.last().unwrap();
                let t = get_type(last_expr, ctx)?;

                arm_types.push(t);
            }

            let first_type = arm_types.first().cloned()?;

            if arm_types.iter().all(|t| *t == first_type) {
                Some(first_type)
            } else {
                Some(Type::Any)
            }
        }

        Expr::VariableRef { name, .. } => {
            println!("Variable name: {}", name);
            if name == "self" {
                ctx.current_struct
                    .as_ref()
                    .map(|name| Type::Istifadeci(name.clone()))
            } else {
                if let Some((_level, sym)) = ctx.lookup_variable_scoped(name) {
                    return Some(sym.typ.clone());
                }
                if ctx.enum_defs.contains_key(name) {
                    return Some(Type::Istifadeci(name.clone()));
                }
                None
            }
        }

        Expr::FieldAccess { target, field, .. } => {
            if let Some(Type::Istifadeci(struct_name)) = get_type(target, ctx) {
                if let Some((fields, _)) = ctx.struct_defs.get(&struct_name) {
                    for (f_name, f_type) in fields {
                        if f_name == field {
                            return Some(f_type.clone());
                        }
                    }
                }
            }
            None
        }
        Expr::StructInit { name, args: _ } => Some(Type::Istifadeci(name.clone())),
        Expr::List(items) => {
            if items.is_empty() {
                return Some(Type::Siyahi(Box::new(Type::Any))); // boş siyahı – tipi bilinmir
            }

            let item_type = get_type(&items[0], ctx)?;

            for item in &items[1..] {
                let t = get_type(item, ctx)?;
                if t != item_type {
                    return Some(Type::Siyahi(Box::new(Type::Any))); // qarışıq tiplər
                }
            }

            Some(Type::Siyahi(Box::new(item_type)))
        }

        Expr::String(s) => {
            if s.len() == 1 {
                Some(Type::Char)
            } else {
                Some(Type::Metn)
            }
        }
        Expr::Number(_) => Some(Type::Integer),
        Expr::Bool(_) => Some(Type::Bool),

        Expr::MethodCall {
            target,
            method,
            args,
        } => {
            let target_type = get_type(target, ctx);

            match target_type {
                Some(Type::Siyahi(_)) => match method.as_str() {
                    "uzunluq" | "boşdur" => Some(Type::Integer),
                    "sırala" | "əks_sırala" => Some(Type::Siyahi(Box::new(Type::Integer))),
                    _ => None,
                },
                Some(Type::Metn) => match method.as_str() {
                    "uzunluq" | "boşdur" => Some(Type::Integer),
                    "böyüt" | "kiçilt" | "kənar_təmizlə" => Some(Type::Metn),
                    "birləşdir" => Some(Type::Metn),
                    "kəs" => Some(Type::Metn),
                    "əvəzlə" => Some(Type::Metn),
                    "böl" => Some(Type::Siyahi(Box::new(Type::Metn))),
                    _ => None,
                },
                Some(Type::Istifadeci(struct_name)) => {
                    if let Some((_fields, methods)) = ctx.struct_defs.get(&struct_name) {
                        for (m_name, params, _body, ret_type) in methods {
                            if m_name == method && args.len() == params.len() - 1 {
                                return ret_type.clone();
                            }
                        }
                    }
                    None
                }
                _ => None,
            }
        }

        Expr::FunctionCall { name, .. } => {
            if let Some(func_info) = ctx.functions.get(name) {
                func_info.return_type.clone()
            } else {
                None
            }
        }
        Expr::Return(inner) => get_type(inner, ctx),
        Expr::BuiltInCall {
            func,
            args,
            resolved_type: _,
        } => match func {
            BuiltInFunction::Print => Some(Type::Metn),
            BuiltInFunction::Len | BuiltInFunction::Number | BuiltInFunction::Sum => {
                Some(Type::Integer)
            }
            BuiltInFunction::Max | BuiltInFunction::Min => Some(Type::Integer),
            BuiltInFunction::Timer => Some(Type::Integer),
            BuiltInFunction::LastWord => Some(Type::Metn),
            BuiltInFunction::Input => Some(Type::Metn),
            BuiltInFunction::Range => {
                if args.len() == 2 {
                    let left = get_type(&args[0], ctx)?;
                    let right = get_type(&args[1], ctx)?;

                    if left == Type::Integer && right == Type::Integer {
                        Some(Type::Siyahi(Box::new(Type::Integer)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        },

        Expr::BinaryOp { left, op, right } => {
            let left_type = get_type(left, ctx)?;
            let right_type = get_type(right, ctx)?;

            if left_type != right_type {
                return None;
            }
            let comparison_ops = ["==", "!=", "<", "<=", ">", ">="];
            let logic_ops = ["&&", "||"];
            if comparison_ops.contains(&op.as_str()) || logic_ops.contains(&op.as_str()) {
                return Some(Type::Bool);
            }

            // Əks halda arifmetik və ya digər operatorlardır – nəticə operandların tipidir
            Some(left_type)
        }

        _ => None,
    }
}
