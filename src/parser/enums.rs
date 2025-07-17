use crate::{
    lexer::Token,
    parser::ast::{EnumDecl, Expr},
};
use color_eyre::eyre::{Result, eyre};
use std::{borrow::Cow, iter::Peekable};

pub fn parse_enum_decl<'a, I>(tokens: &mut Peekable<I>) -> Result<Expr<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    // 1. tip sözü artıq consume edilib
    let name = match tokens.next() {
        Some(Token::Identifier(name)) => Cow::Borrowed((*name).as_str()),
        other => {
            return Err(eyre!(
                "`tip`-dən sonra identifikator gözlənilirdi, tapıldı: {:?}",
                other
            ));
        }
    };

    // 2. Yeni sətrə keçirik
    match tokens.next() {
        Some(Token::Newline) => {}
        other => {
            return Err(eyre!(
                "Enum tərifindən sonra `newline` gözlənilirdi, tapıldı: {:?}",
                other
            ));
        }
    }

    let mut variants = Vec::new();

    loop {
        match tokens.peek() {
            Some(Token::Indent) => {
                tokens.next(); // indent keçirik
            }
            Some(Token::Identifier(var)) => {
                variants.push((*var).as_str());
                tokens.next(); // consume name
            }
            Some(Token::Newline) => {
                tokens.next(); // boş sətir, skip
            }
            Some(Token::Dedent) | Some(Token::End) => {
                tokens.next(); // block bitdi
                break;
            }
            Some(Token::Eof) => break,
            Some(tok) => {
                return Err(eyre!("Enum variantında gözlənilməz token: {:?}", tok));
            }
            None => return Err(eyre!("Enum tərifi gözlənilmədən bitdi")),
        }
    }
    Ok(Expr::EnumDecl(EnumDecl { name, variants }))
}
