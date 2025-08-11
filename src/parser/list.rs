use std::borrow::Cow;

use crate::{lexer::Token, parser::ast::Expr};

pub fn parse_list<'a, I>(tokens: &mut I) -> Expr<'a>
where
    I: Iterator<Item = &'a Token>,
{
    let mut elements = Vec::new();

    for token in tokens.by_ref() {
        match token {
            Token::ListEnd => break,
            Token::StringLiteral(s) => elements.push(Expr::String(s, false)),
            Token::True => elements.push(Expr::Bool(true)),
            Token::False => elements.push(Expr::Bool(false)),
            Token::Float(num) => elements.push(Expr::Float(*num)),
            Token::Number(num) => elements.push(Expr::Number(*num)),
            Token::This => elements.push(Expr::VariableRef {
                name: Cow::Borrowed("self"),
                transpiled_name: Some("self".to_string()),
                symbol: None,
            }),
            Token::Comma => continue,
            Token::Newline => continue,
            Token::Semicolon => continue,
            _ => continue,
        }
    }
    Expr::List(elements)
}
