use crate::{
    lexer::Token,
    parser::{ast::Expr, expression::parse_single_expr, helper::expect_token},
};
use color_eyre::eyre::{Result, eyre};
use std::iter::Peekable;

pub fn parse_loop<'a, I>(tokens: &mut Peekable<I>) -> Result<Expr<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    expect_token(tokens, Token::Loop)?; // `gəz` açar sözü

    // iterable ifadə
    let iterable = parse_single_expr(tokens)?;

    expect_token(tokens, Token::In)?;

    // dəyişən adı gözlənilir
    let var_name = match tokens.next() {
        Some(Token::Identifier(name)) => (*name).as_str(),
        other => return Err(eyre!("Dəyişən adı gözlənilirdi, tapıldı: {:?}", other)),
    };

    expect_token(tokens, Token::Newline)?;
    expect_token(tokens, Token::Indent)?;

    let mut body = Vec::new();

    while let Some(token) = tokens.peek() {
        match token {
            Token::Dedent => {
                tokens.next(); // blok bağlanır
                break;
            }
            Token::Newline => {
                tokens.next(); // boş sətiri keç
            }
            Token::Eof => break,
            _ => {
                let expr = parse_single_expr(tokens)?;
                body.push(expr);

                // Sətir sonundakı nöqtəli vergül/yeni sətiri keç
                while matches!(tokens.peek(), Some(Token::Semicolon | Token::Newline)) {
                    tokens.next();
                }
            }
        }
    }

    Ok(Expr::Loop {
        var_name,
        iterable: Box::new(iterable),
        body,
    })
}
