use std::borrow::Cow;

use crate::{
    errors::ParserError,
    parsing_for::{parse_expression_typed, parse_single_expr_typed},
    shared_ast::Type,
    struct_init::parse_structs_init_typed,
    typed_ast::TypedExpr,
};
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

use crate::{
    ast::Expr,
    expressions::{parse_expression, parse_single_expr},
    struct_init::parse_structs_init,
};

pub fn parse_identifier<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
    s: &'a str,
) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let mut expr = Expr::VariableRef {
        name: Cow::Borrowed(s),
        symbol: None,
    };

    match tokens.peek() {
        Some(Token::ListStart) => {
            tokens.next();
            let index_expr = parse_single_expr(tokens)?;
            if let Some(token) = tokens.next() {
                if *token == Token::ListEnd {
                    Ok(Expr::Index {
                        target: Box::new(expr),
                        index: Box::new(index_expr),
                        target_type: Type::Any,
                    })
                } else {
                    Err(ParserError::ArrayNotClosed(token.clone()))
                }
            } else {
                Err(ParserError::ArrayNotClosed(Token::ListEnd))
            }
        }
        Some(Token::LParen) => {
            tokens.next();
            let mut args = Vec::new();
            loop {
                match tokens.peek() {
                    Some(Token::RParen) => {
                        tokens.next();
                        break;
                    }
                    None => break,
                    _ => {
                        let arg = parse_expression(tokens)?;
                        args.push(arg);

                        match tokens.peek() {
                            Some(Token::Comma) => {
                                tokens.next();
                            }
                            Some(Token::RParen) => {
                                tokens.next();
                                break;
                            }
                            None => return Err(ParserError::RParenNotFound(Token::Eof)),
                            _ => {}
                        }
                    }
                }
            }
            expr = Expr::Call {
                target: None,
                name: s,
                args,
                returned_type: None,
            };
            Ok(expr)
        }
        Some(Token::Operator(op)) if op == "=" => {
            tokens.next();
            let value = parse_expression(tokens)?;

            Ok(Expr::Assignment {
                name: s.into(),
                value: Box::new(value),
                symbol: None,
            })
        }
        Some(Token::Dot) => {
            tokens.next();
            let field_or_method = match tokens.next() {
                Some(Token::Identifier(name)) => (*name).as_str(),
                Some(other) => return Err(ParserError::MethodNameNotFound(other.clone())),
                None => return Err(ParserError::MethodNameNotFound(Token::Eof)),
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
            Ok(expr)
        }
        Some(Token::LBrace) => {
            tokens.next();
            parse_structs_init(tokens, Cow::Borrowed(s))
        }
        Some(_) => Ok(expr),
        None => Err(ParserError::UnexpectedEOF),
    }
}

pub fn parse_identifier_typed<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
    s: &'a str,
) -> Result<TypedExpr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let mut expr = TypedExpr::VariableRef {
        name: Cow::Borrowed(s),
        symbol: None,
        transpiled_name: None,
    };

    match tokens.peek() {
        Some(Token::ListStart) => {
            tokens.next();
            let index_expr = parse_single_expr_typed(tokens)?;
            if let Some(token) = tokens.next() {
                if *token == Token::ListEnd {
                    Ok(TypedExpr::Index {
                        target: Box::new(expr),
                        index: Box::new(index_expr),
                        target_type: Type::Any,
                    })
                } else {
                    Err(ParserError::ArrayNotClosed(token.clone()))
                }
            } else {
                Err(ParserError::ArrayNotClosed(Token::ListEnd))
            }
        }
        Some(Token::LParen) => {
            tokens.next();
            let mut args: Vec<TypedExpr<'a>> = Vec::new();
            loop {
                match tokens.peek() {
                    Some(Token::RParen) => {
                        tokens.next();
                        break;
                    }
                    None => break,
                    _ => {
                        let arg = parse_expression_typed(tokens)?;
                        args.push(arg);

                        match tokens.peek() {
                            Some(Token::Comma) => {
                                tokens.next();
                            }
                            Some(Token::RParen) => {
                                tokens.next();
                                break;
                            }
                            None => return Err(ParserError::RParenNotFound(Token::Eof)),
                            _ => {}
                        }
                    }
                }
            }
            expr = TypedExpr::Call {
                target: None,
                name: s,
                args,
                returned_type: None,
                is_allocator: false,
                transpiled_name: None,
            };
            Ok(expr)
        }
        Some(Token::Operator(op)) if op == "=" => {
            tokens.next();
            let value = parse_expression_typed(tokens)?;

            Ok(TypedExpr::Assignment {
                name: s.into(),
                value: Box::new(value),
                symbol: None,
            })
        }
        Some(Token::Dot) => {
            tokens.next();
            let field_or_method = match tokens.next() {
                Some(Token::Identifier(name)) => (*name).as_str(),
                Some(other) => return Err(ParserError::MethodNameNotFound(other.clone())),
                None => return Err(ParserError::MethodNameNotFound(Token::Eof)),
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
                        returned_type: Some(Type::Any),
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
            Ok(expr)
        }
        Some(Token::LBrace) => {
            tokens.next();
            parse_structs_init_typed(tokens, Cow::Borrowed(s))
        }
        Some(_) => Ok(expr),
        None => Err(ParserError::UnexpectedEOF),
    }
}
