use crate::context::TranspileContext;

pub fn transpile_list_method_call(
    target_code: &str,
    method: &str,
    args_code: &[String],
    is_mutable: bool,
    ctx: &mut TranspileContext,
) -> Result<String, String> {
    match method {
        "əlavə_et" => Ok(format!("try {}.append({});", target_code, args_code[0])),
        "sil" => Ok(format!("_ = {}.swapRemove({});", target_code, args_code[0])),
        "sıralı_sil" => Ok(format!(
            "_ = {}.orderedRemove({});",
            target_code, args_code[0]
        )),
        "sırala" => Ok(format!(
            "std.mem.sort(usize, {0}.items, {{ }}, std.sort.asc(usize));",
            target_code
        )),
        "əks_sırala" => Ok(format!(
            "std.mem.sort(usize, {0}.items, {{ }}, std.sort.desc(usize));",
            target_code
        )),
        "uzunluq" => {
            if is_mutable {
                Ok(format!("{}.items.len", target_code))
            } else {
                Ok(format!("{}.len", target_code))
            }
        }
        "axtar" => {
            ctx.is_find_method = true;
            Ok(format!(" find_index(&{0}, {1})", target_code, args_code[0]))
        }
        "boşdur" => Ok(format!("{}.is_empty()", target_code)),
        _ => Err("dəstəklənmir".to_string()), // bu hissə parserdə yoxlanacaqsa, buranı da istəsən saxlamaya bilərik.
    }
}
