use crate::{
    context::{Symbol, TranspileContext},
    parser::{Expr, ast::Type, types::get_type},
    validate_expr,
};

pub fn validate_decl(
    name: &str,
    typ: &Option<Type>,
    value: &mut Expr,
    is_mutable: bool,
    ctx: &mut TranspileContext,
    message: &mut dyn FnMut(&str),
) -> Result<(), String> {
    message(&format!(
        "{} yaradılır: '{}'",
        if is_mutable { "Dəyişən" } else { "Sabit" },
        name
    ));

    let inferred = match get_type(value, ctx) {
        Some(t) => t,
        None => {
            if typ.is_none() {
                return Err(format!(
                    "'{}' üçün tip təyin edilə bilmədi və göstərilməyib",
                    name
                ));
            }

            if let Some(Type::Istifadeci(enum_name)) = typ {
                if let Expr::VariableRef {
                    name: variant_name, ..
                } = value
                {
                    if let Some(variants) = ctx.enum_defs.get(enum_name) {
                        if variants.contains(variant_name) {
                            Type::Istifadeci(enum_name.clone())
                        } else {
                            return Err(format!(
                                "'{}' enum tipi üçün variant '{}' mövcud deyil",
                                enum_name, variant_name
                            ));
                        }
                    } else {
                        return Err(format!("Enum '{}' tapılmadı", enum_name));
                    }
                } else {
                    return Err("Dəyər enum variantı deyil".to_string());
                }
            } else {
                return Err(format!("'{}' üçün tip təyin edilə bilmədi", name));
            }
        }
    };

    let declared = typ.clone().unwrap_or_else(|| inferred.clone());

    if inferred != declared {
        return Err(format!(
            "{} üçün tip uyğunsuzluğu: gözlənilən {:?}, tapılan {:?}",
            name, declared, inferred
        ));
    }

    ctx.declare_variable(
        name.to_string(),
        Symbol {
            typ: declared,
            is_mutable,
            is_used: false,
            is_pointer: false,
            source_location: None,
        },
    );

    validate_expr(&mut value.clone(), ctx, message)
}

pub fn find_used_outer_mutable_vars(body: &[Expr], ctx: &TranspileContext) -> Vec<String> {
    let mut used = Vec::new();

    fn visit(expr: &Expr, used: &mut Vec<String>, ctx: &TranspileContext) {
        match expr {
            Expr::VariableRef { name, .. } | Expr::Assignment { name, .. } => {
                if let Some((_lvl, symbol)) = ctx.lookup_variable_scoped(name) {
                    if symbol.is_mutable && !symbol.is_pointer {
                        used.push(name.clone());
                    }
                }
            }
            _ => {
                for child in expr.children() {
                    visit(child, used, ctx);
                }
            }
        }
    }

    for expr in body {
        visit(expr, &mut used, ctx);
    }

    used
}
