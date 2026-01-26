use std::borrow::Cow;

use tokenizer::{iterator::Tokens, tokens::Token};

use crate::ast::Expr;

pub fn parse_list<'a>(tokens: &mut Tokens) -> Expr<'a>
{
    let mut elements = Vec::new();

    for token in tokens.by_ref() {
        match token.token {
            Token::ListEnd => break,
            Token::StringLiteral(s) => elements.push(Expr::String(s)),
            Token::True => elements.push(Expr::Bool(true)),
            Token::False => elements.push(Expr::Bool(false)),
            Token::Float(num) => elements.push(Expr::Float(num)),
            Token::Number(num) => elements.push(Expr::Number(num)),
            Token::This => elements.push(Expr::VariableRef {
                name: Cow::Borrowed("self"),
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
