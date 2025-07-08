use crate::{
    Symbol, ValidatorContext,
    parser::{Expr, ast::Type, types::get_type},
    validate_expr,
};

pub fn validate_decl(
    name: &str,
    typ: &Option<Type>,
    value: &mut Expr,
    is_mutable: bool,
    ctx: &mut ValidatorContext,
    message: &mut dyn FnMut(&str),
) -> Result<(Type, String), String> {
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
    let transpile_name = sanitize_name(name);
    ctx.declare_variable(
        name.to_string(),
        Symbol {
            typ: declared.clone(),
            is_mutable,
            is_used: false,
            is_pointer: false,
            source_location: None,
            transpile_name: transpile_name.clone(),
        },
    );

    validate_expr(value, ctx, message)?;

    Ok((declared, transpile_name))
}

pub fn sanitize_name(name: &str) -> String {
    let mut result = String::from("az_");

    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            result.push(ch);
        } else {
            result.push_str(&format!("_u{:x}", ch as u32));
        }
    }

    result
}
