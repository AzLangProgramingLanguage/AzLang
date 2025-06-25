use crate::context::{Parameter, Symbol, TranspileContext};
use crate::helper::{find_used_outer_mutable_vars, validate_decl};
use crate::lexer::Token;
use crate::parser::Expr;
use crate::parser::ast::{BuiltInFunction, EnumDecl, TemplateChunk, Type};
use crate::parser::types::get_type;

pub fn validate_expr(
    expr: &mut Expr,
    ctx: &mut TranspileContext,
    message: &mut dyn FnMut(&str),
) -> Result<(), String> {
    match expr {
        Expr::EnumDecl(EnumDecl { name, variants }) => {
            message(&format!("Enum tərifi yoxlanılır: '{}'", name));

            if ctx.enum_defs.contains_key(name) {
                return Err(format!("Enum '{}' artıq mövcuddur", name));
            }

            ctx.enum_defs.insert(name.clone(), variants.clone());
        }

        Expr::Match(match_expr) => {
            validate_expr(&mut match_expr.target, ctx, message)?;
            message(&format!("Match ifadəsinin tipi: {:?}", match_expr.target));
            let target_type = get_type(&match_expr.target, ctx)
                .ok_or_else(|| "Match ifadəsində target tip təyin edilə bilmədi".to_string())?;

            if let Type::Istifadeci(enum_name) = &target_type {
                // Enum üçün xüsusi yoxlama
                let enum_variants = ctx
                    .enum_defs
                    .get(enum_name)
                    .ok_or_else(|| format!("Match üçün '{}' enum tərifi tapılmadı", enum_name))?
                    .clone();

                for (variant_token, expr_block) in &mut match_expr.arms {
                    // Token-dən string çıxarırıq
                    let variant_name = match variant_token {
                        Token::Identifier(s) => s,
                        Token::Underscore => "_",
                        _ => {
                            return Err(format!(
                                "Enum match üçün uyğun olmayan pattern: {:?}",
                                variant_token
                            ));
                        }
                    };

                    message(&format!("Match variantı yoxlanılır: '{}'", variant_name));

                    if variant_name != "_" && !enum_variants.contains(&variant_name.to_string()) {
                        return Err(format!(
                            "'{}' enum variantı '{}' tapılmadı",
                            enum_name, variant_name
                        ));
                    }

                    for expr in expr_block.iter_mut() {
                        validate_expr(expr, ctx, message)?;
                    }
                }
            } else {
                // Sadə tiplər üçün: rəqəmlər, sətirlər və `_`
                for (pattern_token, expr_block) in match_expr.arms.iter_mut() {
                    match pattern_token {
                        Token::Number(_) | Token::Underscore => {
                            // keçərli — əlavə yoxlamaya ehtiyac yoxdur
                        }
                        Token::Identifier(s) if s == "_" => {
                            // "_" stringi olan identifier-ə də icazə veririk
                        }
                        Token::StringLiteral(s) => {
                            if s.len() != 1 {
                                return Err(format!(
                                    "String literal match üçün yalnız 1 simvol gözlənilirdi, tapıldı: {}",
                                    s
                                ));
                            }
                        }
                        Token::Identifier(s) => {
                            // Enum olmayan match-da `Identifier` uyğun deyil
                            return Err(format!(
                                "Enum olmayan match üçün qeyri-qanuni identifier: '{}'",
                                s
                            ));
                        }
                        other => {
                            return Err(format!("Match üçün tanınmayan token: {:?}", other));
                        }
                    }

                    for expr in expr_block.iter_mut() {
                        validate_expr(expr, ctx, message)?;
                    }
                }
            }
        }

        Expr::ConstantDecl { name, typ, value } => {
            validate_decl(name, typ, value, false, ctx, message)?;
        }

        Expr::MutableDecl { name, typ, value } => {
            validate_decl(name, typ, value, true, ctx, message)?;
        }

        Expr::Assignment {
            name,
            value,
            symbol,
        } => {
            message(&format!("Mənimsətmə yoxlanılır: {} = ...", name));

            let (_level, sym) = ctx
                .lookup_variable_scoped(name)
                .ok_or_else(|| format!("Dəyişən '{}' elan edilməyib", name))?;

            if !sym.is_mutable {
                return Err(format!("Sabit '{}' dəyişdirilə bilməz", name));
            }

            let value_type = get_type(value, ctx)
                .ok_or_else(|| format!("{} üçün tip təyin edilə bilmədi", name))?;

            if value_type != sym.typ {
                return Err(format!(
                    "Tip uyğunsuzluğu: '{}' üçün {:?} gözlənilirdi, lakin {:?} tapıldı",
                    name, sym.typ, value_type
                ));
            }

            // ✅ AST içində symbol-u güncəllə
            *symbol = Some(sym.clone());

            validate_expr(value, ctx, message)?;
        }

        Expr::StructDef {
            name,
            fields,
            methods,
        } => {
            message(&format!("Struktur elan edilir: '{}'", name));

            // Eyni adda struktur varsa, xəta qaytar
            if ctx.struct_defs.contains_key(name) {
                return Err(format!("Struktur '{}' artıq mövcuddur", name));
            }

            // Sadəcə sahələri yadda saxlayırıq — metodları ayrıca saxlamaq istəmirsənsə
            ctx.struct_defs
                .insert(name.clone(), (fields.clone(), methods.clone()));
        }

        Expr::StructInit { name, args } => {
            message(&format!("Struktur yaradılır: '{}'", name));

            let (struct_fields, _methods) = ctx
                .struct_defs
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Struktur '{}' tapılmadı", name))?;

            if struct_fields.len() != args.len() {
                return Err(format!(
                    "'{}' strukturu üçün {} arqument gözlənilirdi, lakin {} verildi",
                    name,
                    struct_fields.len(),
                    args.len()
                ));
            }

            for (i, (field_name, expected_type)) in struct_fields.iter().enumerate() {
                let arg_expr = &mut args[i]; // ✅ `&mut Expr`
                let actual_type = get_type(arg_expr, ctx).ok_or_else(|| {
                    format!("'{}' sahəsi üçün tip təyin edilə bilmədi", field_name)
                })?;

                validate_expr(arg_expr, ctx, message)?; // ✅ Doğru borrow

                if &actual_type != expected_type {
                    return Err(format!(
                        "'{}' sahəsi üçün tip uyğunsuzluğu: gözlənilən {:?}, tapılan {:?}",
                        field_name, expected_type, actual_type
                    ));
                }
            }
        }

        Expr::TemplateString(chunks) => {
            message("Template string yoxlanılır");
            for chunk in chunks.iter_mut() {
                match chunk {
                    TemplateChunk::Literal(_) => {}
                    TemplateChunk::Expr(expr) => {
                        validate_expr(expr, ctx, message)?;
                    }
                }
            }
        }

        Expr::FieldAccess { target, field } => {
            validate_expr(target, ctx, message)?;

            let target_type = get_type(target, ctx)
                .ok_or_else(|| "FieldAccess üçün tip təyin edilə bilmədi".to_string())?;

            let struct_name = if let Type::Istifadeci(name) = target_type {
                name
            } else {
                return Err("Sahəyə yalnız struktur növü üzərindən çıxış edilə bilər".to_string());
            };

            let (struct_fields, _methods) = ctx
                .struct_defs
                .get(&struct_name)
                .ok_or_else(|| format!("Struktur '{}' tapılmadı", struct_name))?;

            let found = struct_fields.iter().any(|(f, _)| f == field);

            if !found {
                return Err(format!(
                    "'{}' strukturu sahəyə sahib deyil: '{}'",
                    struct_name, field
                ));
            }
        }

        Expr::If {
            condition,
            then_branch,
            else_branch,
        } => {
            message("Şərtə baxılır (if)");

            validate_expr(condition, ctx, message)?;

            let cond_type =
                get_type(condition, ctx).ok_or("İf şərtinin tipi müəyyən edilə bilmədi")?;

            if cond_type != Type::Bool {
                return Err(format!(
                    "İf şərti `bool` olmalıdır, tapıldı: {:?}",
                    cond_type
                ));
            }

            for expr in then_branch {
                validate_expr(expr, ctx, message)?;
            }

            for expr in else_branch {
                validate_expr(expr, ctx, message)?;
            }
        }

        Expr::ElseIf {
            condition,
            then_branch,
        } => {
            message("Şərtə baxılır (else if)");

            validate_expr(condition, ctx, message)?;

            let cond_type =
                get_type(condition, ctx).ok_or("Else if şərtinin tipi müəyyən edilə bilmədi")?;

            if cond_type != Type::Bool {
                return Err(format!(
                    "`else if` şərti `bool` olmalıdır, tapıldı: {:?}",
                    cond_type
                ));
            }

            for expr in then_branch {
                validate_expr(expr, ctx, message)?;
            }
        }

        Expr::Else { then_branch } => {
            message("Else hissəsi yoxlanılır");

            for expr in then_branch {
                validate_expr(expr, ctx, message)?;
            }
        }

        Expr::BinaryOp { left, op, right } => {
            message(&format!("İki tərəfli əməliyyat: {:?}", op));

            validate_expr(left, ctx, message)?;
            validate_expr(right, ctx, message)?;

            let left_type = get_type(left, ctx);
            let right_type = get_type(right, ctx);

            if left_type != right_type {
                return Err(format!(
                    "Binary `{}` operatorunda tip uyğunsuzluğu: {:?} və {:?}",
                    op, left_type, right_type
                ));
            }
        }

        Expr::BuiltInCall {
            func,
            args,
            resolved_type: _,
        } => {
            message(&format!("Daxili funksiya çağırılır: {:?}", func));

            for arg in args.iter_mut() {
                validate_expr(arg, ctx, message)?;
            }

            if *func == BuiltInFunction::Sum {
                if let Some(t) = get_type(&args[0], ctx) {
                    match t {
                        Type::Siyahi(inner) if *inner == Type::Integer => {}
                        _ => {
                            return Err(
                                "sum funksiyası yalnız ədəd tipli siyahı qəbul edir".to_string()
                            );
                        }
                    }
                }
            }
        }

        Expr::MethodCall {
            target,
            method,
            args,
        } => {
            validate_expr(target, ctx, message)?;
            for arg in args.iter_mut() {
                validate_expr(arg, ctx, message)?;
            }
            println!("Target: {:?}", target); //Target: VariableRef { name: "adam", symbol: None }
            let target_type = get_type(target, ctx)
                .ok_or_else(|| "MethodCall üçün tip təyin edilə bilmədi".to_string())?;

            validate_method_call(&target_type, method, args, ctx)?;
        }

        Expr::FunctionCall {
            name,
            args,
            resolved_params,
            return_type,
        } => {
            message(&format!("Funksiya çağırışı yoxlanılır: {}", name));

            // Funksiyanı konteksdən tap
            let func = ctx
                .lookup_function(name)
                .ok_or_else(|| format!("Funksiya tapılmadı: '{}'", name))?;

            println!("Func: {:?}", func);

            // Pointer parametrlər üçün avtomatik dəyişən axtarışı
            for param in &func.parameters {
                if param.is_pointer {
                    // Əgər artıq args-da varsa keç
                    if args.iter().any(
                        |arg| matches!(arg, Expr::VariableRef { name: n, .. } if n == &param.name),
                    ) {
                        continue;
                    }
                    // Kontekstdən həmin dəyişəni tap
                }
            }

            // Artıq `args` doludur → onları yoxlayırıq
            for arg in args.iter_mut() {
                validate_expr(arg, ctx, message)?;
            }

            *resolved_params = func.parameters.clone(); // funksiya parametrləri ilə eşlə
            *return_type = func.return_type.clone(); // geri dönüş tipini də təyin et
        }

        Expr::FunctionDef {
            name,
            params,
            body,
            return_type: _,
        } => {
            message(&format!("Funksiya tərifi: {}", name));
            // 💡 Əvvəlcə scope daxilində olmayan `mutable` dəyişənləri tapırıq:
            let outer_used_vars = find_used_outer_mutable_vars(body, ctx);

            for outer_name in outer_used_vars {
                // Əgər artıq parametr siyahısında varsa, atla
                if params.iter().any(|p| p.name == outer_name) {
                    continue;
                }

                if let Some((_level, symbol)) = ctx.lookup_variable_scoped(&outer_name) {
                    if symbol.is_mutable {
                        params.push(Parameter {
                            name: outer_name.clone(),
                            typ: symbol.typ.clone(),
                            is_mutable: true,
                            is_pointer: true,
                        });
                    }
                }
            }

            ctx.push_scope();

            // ✅ Parametrləri kontekstə tanıt
            for param in params.iter() {
                message(&format!(
                    "Parametr əlavə edilir: {}: {:?}",
                    param.name, param.typ
                ));

                let symbol = Symbol {
                    typ: param.typ.clone(),
                    is_mutable: param.is_mutable,
                    is_used: false,
                    is_pointer: true,
                    source_location: None,
                };

                ctx.declare_variable(param.name.clone(), symbol);
            }

            for stmt in body.iter_mut() {
                validate_expr(stmt, ctx, message)?;
            }
            ctx.update_function_body_and_params(name, params.clone(), body.clone());

            ctx.pop_scope();
            return Ok(());
        }

        Expr::Loop {
            iterable,
            var_name,
            body,
        } => {
            message("Dəmir Əmi dövrü yoxlayır...");

            validate_expr(iterable, ctx, message)?;

            let iterable_type = get_type(iterable, ctx).ok_or_else(|| {
                "Dövr üçün istifadə edilən obyektin tipi təyin edilə bilmədi".to_string()
            })?;

            if let Type::Siyahi(inner) = iterable_type {
                let symbol = Symbol {
                    typ: *inner,
                    is_mutable: false,
                    is_used: false,
                    is_pointer: false,
                    source_location: None,
                };

                ctx.declare_variable(var_name.clone(), symbol);
                message(&format!(
                    "Dəyişən `{}` siyahıdan götürülərək elan edildi",
                    var_name
                ));
            } else {
                return Err(
                    "Dövr üçün istifadə edilən obyekt siyahı (`list`) olmalıdır".to_string()
                );
            }

            for expr in body {
                validate_expr(expr, ctx, message)?;
            }
        }

        Expr::Return(expr) => {
            message("Dəmir Əmi return ifadəsini yoxlayır...");
            validate_expr(expr, ctx, message)?;
        }

        Expr::List(items) => {
            message("Dəmir Əmi siyahını yoxlayır...");

            if items.is_empty() {
                message("Boş siyahı tapıldı, problem yoxdur.");
                return Ok(());
            }

            // Burada `get_type`-ə readonly gərəkdir, o halda `items[0]` ola bilər
            let first_type = get_type(&items[0], ctx).ok_or_else(|| {
                let msg = "Siyahının ilk elementi üçün tip təyin edilə bilmədi";
                message(msg);
                msg.to_string()
            })?;

            // ⛏️ `iter_mut` → dəyişmək üçün
            for item in items.iter_mut().skip(1) {
                let t = get_type(item, ctx).ok_or_else(|| {
                    let msg = "Siyahı elementi üçün tip təyin edilə bilmədi";
                    message(msg);
                    msg.to_string()
                })?;

                if t != first_type {
                    let msg = format!(
                        "Siyahı daxilində tip uyğunsuzluğu: {:?} və {:?}",
                        first_type, t
                    );
                    message(&msg);
                    return Err(msg);
                }

                validate_expr(item, ctx, message)?;
            }
        }

        Expr::Break => {}
        Expr::Continue => {}

        Expr::Index { target, index } => {
            message("Dəmir Əmi indeksləmə əməliyyatını yoxlayır...");
            validate_expr(target, ctx, message)?;
            validate_expr(index, ctx, message)?;
        }

        Expr::VariableRef { name, symbol } => {
            message(&format!("Dəmir Əmi dəyişənə baxır: `{}`", name));

            if let Some((_level, found_symbol)) = ctx.lookup_variable_scoped(name) {
                *symbol = Some(found_symbol); // 🧠 AST zənginləşdirilir!
            } else {
                // Enum variant olub olmadığını yoxla
                let mut found_in_enum = false;
                for (_enum_name, variants) in &ctx.enum_defs {
                    if variants.contains(name) {
                        found_in_enum = true;
                        break;
                    }
                }

                if !found_in_enum {
                    let msg = format!("Dəyişən '{}' istifadə olunmadan əvvəl elan edilməyib", name);
                    message(&msg);
                } else {
                }
            }
        }

        Expr::String(_) | Expr::Bool(_) | Expr::Number(_) => {}
    }

    Ok(())
}

fn validate_method_call(
    target_type: &Type,
    method: &str,
    args: &[Expr],
    ctx: &TranspileContext,
) -> Result<(), String> {
    match target_type {
        Type::Metn | Type::Siyahi(_) => {
            // Built-in metodların yoxlanması
            match method {
                "əlavə_et" | "sil" | "sıralı_sil" => {
                    if args.len() != 1 {
                        return Err(format!("{} metodu yalnız 1 arqument qəbul edir", method));
                    }
                }

                "sırala" | "əks_sırala" | "uzunluq" | "boşdur" => {
                    if !args.is_empty() {
                        return Err(format!("{} metodu arqumentsiz olmalıdır", method));
                    }
                }

                "cəm" | "sum" => {
                    if args.len() != 1 {
                        return Err(format!("{} metodu yalnız 1 arqument qəbul edir", method));
                    }
                }

                "aralıq" | "range" => {
                    if args.len() != 2 {
                        return Err(format!("{} metodu yalnız 2 arqument qəbul edir", method));
                    }
                }

                "böyüt" | "kiçilt" | "kənar_təmizlə" => {
                    if !args.is_empty() {
                        return Err(format!("{} metodu arqumentsiz olmalıdır", method));
                    }
                }

                "əvəzlə" | "kəs" => {
                    if args.len() != 2 {
                        return Err(format!("{} metodu 2 arqument qəbul edir", method));
                    }
                }

                "birləşdir" | "böl" => {
                    if args.len() != 1 {
                        return Err(format!("{} metodu yalnız 1 arqument qəbul edir", method));
                    }
                }

                _ => return Err(format!("Dəstəklənməyən metod: {}", method)),
            }

            Ok(())
        }

        Type::Istifadeci(struct_name) => {
            // İstifadəçi tərəfindən təyin olunan metodları tap
            let (_, methods) = ctx
                .struct_defs
                .get(struct_name)
                .ok_or_else(|| format!("Struktur tapılmadı: {}", struct_name))?;

            for (method_name, params, _body, _ret_type) in methods {
                if method_name == method {
                    // `self` avtomatik verilir, ona görə 1 çıxılır
                    let expected_arg_count = params.iter().filter(|p| p.name != "self").count();
                    if args.len() != expected_arg_count {
                        return Err(format!(
                            "'{}' metodu {} arqument qəbul edir, amma {} verildi",
                            method,
                            expected_arg_count,
                            args.len()
                        ));
                    } else {
                        return Ok(());
                    }
                }
            }

            Err(format!(
                "'{}' strukturu belə bir metoda sahib deyil: '{}'",
                struct_name, method
            ))
        }

        _ => Err(format!("Tip metodları dəstəkləmir: {:?}", target_type)),
    }
}
