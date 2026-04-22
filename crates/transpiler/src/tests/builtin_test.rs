#[cfg(test)]
mod tests {
    use std::{rc::Rc, result};

    use crate::{TranspileContext, builtin::print};
    use parser::{
        ast::{Expr, Symbol, TemplateChunk},
        shared_ast::Type,
    };
    #[test]
    fn print_bool_test() {
        let mut ctx = TranspileContext::default();
        let expr = Expr::Bool(true);
        let expr2 = Expr::Bool(false);
        let result = print::transpile_print(expr, &mut ctx);
        let result2 = print::transpile_print(expr2, &mut ctx);
        assert_eq!(result, "try std.fs.File.stdout().writeAll(\"doğrudur\n\")");
        assert_eq!(
            result2,
            "try std.fs.File.stdout().writeAll(\"yanlışdır\n\")"
        )
    }
    #[test]
    fn variable_print_test() {
        let mut ctx = TranspileContext::default();
        let expr = Expr::VariableRef {
            name: "a".to_string(),
            symbol: Some(Symbol {
                is_pointer: false,
                typ: Type::Integer,
                is_mutable: false,
                is_used: true,
                is_changed: false,
            }),
        };
        let result = print::transpile_print(expr, &mut ctx);
        assert_eq!(result, "std.debug.print(\"{}\\n\",.{a})")
    }
    #[test]
    fn string_print_test() {
        let mut ctx = TranspileContext::default();
        let result = print::transpile_print(Expr::String("Hello".to_string()), &mut ctx);
        assert_eq!(result, "try std.fs.File.stdout().writeAll(\"Hello\\n\")")
    }
    #[test]
    fn template_string_print_test() {
        let mut ctx = TranspileContext::default();
        let chunk = TemplateChunk::Literal("Hello".to_string());
        let expr = Expr::TemplateString(vec![chunk]);
        let result = print::transpile_print(expr, &mut ctx);
        assert_eq!(result, "std.debug.print(\"Hello\\n\",.{})")
    }
}
