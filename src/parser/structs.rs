use color_eyre::eyre::{Result, eyre};
use peekmore::PeekMoreIterator;

use crate::{
    lexer::Token,
    parser::{ast::Expr, expression::parse_single_expr},
};

pub fn parse_structs_init<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
    name: &'a str,
) -> Result<Expr<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    let mut args = Vec::new();
    loop {
        match tokens.peek() {
            Some(Token::RBrace) => {
                tokens.next();
                break;
            }
            None => break,
            _ => {
                let arg = parse_single_expr(tokens)?;
                args.push(arg);
                tokens.next();
                if let Some(Token::Comma) = tokens.peek() {
                    tokens.next();
                } else {
                    if !matches!(tokens.peek(), Some(Token::RBrace)) {
                        return Err(eyre!(
                            "Struct init argümentləri arasında ',' və ya '}}' gözlənilirdi"
                        ));
                    }
                }
            }
        }
    }

    Ok(Expr::StructInit { name, args })
}
