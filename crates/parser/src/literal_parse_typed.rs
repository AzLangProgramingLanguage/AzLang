use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

use crate::{
    errors::ParserError, list::parse_list_typed, parsing_for::parse_single_expr_typed,
    shared_ast::Type, typed_ast::TypedExpr,
};

pub fn literals_parse_typed<'a, I>(
    token: &'a Token,
    tokens: &mut PeekMoreIterator<I>,
) -> Result<TypedExpr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let mut expr = match token {
        Token::StringLiteral(s) => TypedExpr::String(s, false),
        Token::Number(num) => TypedExpr::Number(*num),
        Token::Float(num) => TypedExpr::Float(*num),
        Token::ListStart => parse_list_typed(tokens),
        /* TODO: Buraya baxarsan çünki literal parse etmesi lazım  */
        _ => return Err(ParserError::UnexpectedToken(token.clone())),
    };

    while let Some(Token::Dot) = tokens.peek() {
        tokens.next();

        let field_or_method = match tokens.next() {
            Some(Token::Identifier(name)) => (*name).as_str(),
            None => {
                return Err(ParserError::UnexpectedEOF);
            }
            Some(other) => {
                return Err(ParserError::MethodNameNotFound(other.clone()));
            }
        };

        match tokens.peek() {
            Some(Token::LParen) => {
                tokens.next();
                let mut args = Vec::new();

                while let Some(token) = tokens.peek() {
                    match token {
                        Token::RParen => {
                            tokens.next();
                            break;
                        }
                        Token::Comma => {
                            tokens.next();
                        }
                        _ => {
                            let arg = parse_single_expr_typed(tokens)?;
                            args.push(arg);
                        }
                    }
                }

                expr = TypedExpr::Call {
                    target: Some(Box::new(expr)),
                    name: field_or_method,
                    args,
                    returned_type: None,
                    is_allocator: false,
                    transpiled_name: None,
                };
            }
            _ => {
                expr = TypedExpr::Index {
                    target: Box::new(expr),
                    index: Box::new(TypedExpr::String(field_or_method, false)),
                    target_type: Type::Any,
                };
            }
        }
    }

    Ok(expr)
}
