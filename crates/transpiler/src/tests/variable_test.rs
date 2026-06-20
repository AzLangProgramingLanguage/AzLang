#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use parser::{
        ast::{Atom, Expr, Statement},
        shared_ast::{StringEnum, Type},
    };

    use crate::{TranspileContext, transpile::transpile_stmt};

    #[test]
    fn variable_decl_num_test() {
        let mut ctx = TranspileContext::default();
        let var_statement = Statement::Decl {
            name: Atom::from("a"),
            typ: Rc::new(Type::Natural),
            is_mutable: true,
            value: Box::new(Expr::Number(1)),
        };
        let const_statement = Statement::Decl {
            name: Atom::from("a"),
            typ: Rc::new(Type::Natural),
            is_mutable: false,
            value: Box::new(Expr::Number(1)),
        };

        assert_eq!(
            transpile_stmt(const_statement, &mut ctx),
            String::from("const a: i64 = 1")
        );
        assert_eq!(
            transpile_stmt(var_statement, &mut ctx),
            String::from("var a: i64 = 1")
        )
    }
    #[test]
    fn variable_decl_str_test() {
        let mut ctx = TranspileContext::default();
        let var_statement = Statement::Decl {
            name: Atom::from("a"),
            typ: Rc::new(Type::String(StringEnum::LiteralString)),
            is_mutable: true,
            value: Box::new(Expr::String(Atom::from("Salam"))),
        };
        let const_statement = Statement::Decl {
            name: Atom::from("a"),
            typ: Rc::new(Type::String(StringEnum::LiteralConstString)),
            is_mutable: false,
            value: Box::new(Expr::String(Atom::from("S"))),
        };

        assert_eq!(
            transpile_stmt(const_statement, &mut ctx),
            String::from("const a: []const u8 = \"S\"")
        );
        assert_eq!(
            transpile_stmt(var_statement, &mut ctx),
            String::from("var a: []u8 = \"Salam\"")
        )
    }
}
