use peekmore::PeekMoreIterator;

use color_eyre::eyre::{Result, eyre};

use crate::{
    lexer::Token,
    parser::{
        ast::{Expr, Type},
        expression::parse_single_expr,
    },
};

pub fn skip_newlines<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<()>
where
    I: Iterator<Item = &'a Token>,
{
    while matches!(tokens.peek(), Some(Token::Newline)) {
        tokens.next();
    }
    Ok(())
}

pub fn expect_token<'a, I>(tokens: &mut PeekMoreIterator<I>, expected: Token) -> Result<()>
where
    I: Iterator<Item = &'a Token>,
{
    match tokens.next() {
        Some(t) if *t == expected => Ok(()),
        other => Err(eyre!("Gözlənilirdi {:?}, tapıldı {:?}", expected, other)),
    }
}

pub fn literals_parse<'a, I>(token: &'a Token, tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    // İlk literalı parse et
    let mut expr = match token {
        Token::StringLiteral(s) => Expr::String(s, false),
        Token::Number(num) => Expr::Number(*num),
        Token::Float(num) => Expr::Float(*num),
        _ => return Err(eyre!("Literal gözlənilirdi, alındı {:?}", token)),
    };

    // Dot-chaining üçün loop
    while let Some(Token::Dot) = tokens.peek() {
        tokens.next(); // consume '.'

        let field_or_method = match tokens.next() {
            Some(Token::Identifier(name)) => (*name).as_str(),
            other => {
                return Err(eyre!(
                    "Metod və ya sahə adı gözlənilirdi, alındı {:?}",
                    other
                ));
            }
        };

        match tokens.peek() {
            Some(Token::LParen) => {
                tokens.next(); // consume '('
                let mut args = Vec::new();

                while let Some(token) = tokens.peek() {
                    match token {
                        Token::RParen => {
                            tokens.next(); // consume ')'
                            break;
                        }
                        Token::Comma => {
                            tokens.next();
                        }
                        _ => {
                            let arg = parse_single_expr(tokens)?;
                            args.push(arg);
                        }
                    }
                }

                expr = Expr::Call {
                    target: Some(Box::new(expr)),
                    name: field_or_method,
                    args,
                    returned_type: Some(Type::Any),
                    is_allocator: false,
                    transpiled_name: None,
                };
            }
            _ => {
                expr = Expr::Index {
                    target: Box::new(expr),
                    index: Box::new(Expr::String(field_or_method, false)),
                    target_type: Type::Any,
                };
            }
        }
    }

    Ok(expr)
}
