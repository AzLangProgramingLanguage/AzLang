use crate::context::TranspileContext;

pub fn generate_imports(ctx: &TranspileContext) -> String {
    let mut imports = String::new();
    if !ctx.imports.contains("const std = @import(\"std\");") {
        imports.push_str("const std = @import(\"std\");\n");
    }
    for imp in &ctx.imports {
        if imp != "const std = @import(\"std\");" {
            imports.push_str(imp);
            imports.push('\n');
        }
    }
    imports
}
