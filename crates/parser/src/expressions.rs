use crate::{
    ast::{Expr, Operation},
    binary_op::parse_expression,
    builtin::parse_builtin,
    decl::parse_decl,
    errors::ParserError,
    function::parse_function_def,
    helpers::expect_token,
    identifier::parse_identifier,
    literal_parse::literals_parse,
    r#loop::parse_loop,
    shared_ast::Type,
    template::parse_template_string_expr,
};
use tokenizer::{
    iterator::{SpannedToken, Tokens},
    tokens::Token,
};

pub fn parse_expression_block<'a>(tokens: &mut Tokens) -> Result<Vec<Expr<'a>>, ParserError> {
    let mut ast = Vec::new();

    while let Some(token) = tokens.peek() {
        match token.token {
            Token::Newline | Token::Semicolon | Token::Indent => {
                tokens.next();
                continue;
            }

            Token::Import => {
                tokens.nth(2);
            }
            Token::StringLiteral(_) | Token::Number(_) | Token::Float(_) => {
                return Err(ParserError::NotUserDirectValue);
            }
            Token::Eof => {
                break;
            }
            _ => {
                let expr = parse_expression(tokens)?;
                ast.push(expr);
            }
        }
    }
    Ok(ast)
}

pub fn parse_single_expr<'a>(tokens: &mut Tokens) -> Result<Expr<'a>, ParserError> {
    let token = tokens.next().ok_or(ParserError::UnexpectedEOF)?;
    match token {
        SpannedToken {
            token: Token::ConstantDecl,
            ..
        } => {
            return parse_decl(tokens, false);
        }
        SpannedToken {
            token: Token::MutableDecl,
            ..
        } => {
            return parse_decl(tokens, true);
        }

        SpannedToken {
            token: Token::StringLiteral(_),
            ..
        } => literals_parse(token, tokens),
        SpannedToken {
            token: Token::Float(_num),
            ..
        } => literals_parse(token, tokens),
        SpannedToken {
            token: Token::Number(_num),
            ..
        } => literals_parse(token, tokens),
        SpannedToken {
            token: Token::True, ..
        } => Ok(Expr::Bool(true)),
        SpannedToken {
            token: Token::False,
            ..
        } => Ok(Expr::Bool(false)),

        SpannedToken {
            token: Token::Break,
            ..
        } => Ok(Expr::Break),
        SpannedToken {
            token: Token::Comment(s),
            ..
        } => Ok(Expr::Comment(s)),
        SpannedToken {
            token: Token::Return,
            ..
        } => {
            let returned_value = parse_expression(tokens)?;
            Ok(Expr::Return(Box::new(returned_value)))
        }
        SpannedToken {
            token: Token::Continue,
            ..
        } => Ok(Expr::Continue),

        SpannedToken {
            token: Token::Print,
            ..
        }
        | SpannedToken {
            token: Token::Input,
            ..
        }
        | SpannedToken {
            token: Token::Len, ..
        }
        | SpannedToken {
            token: Token::NumberFn,
            ..
        }
        | SpannedToken {
            token: Token::Sum, ..
        }
        | SpannedToken {
            token: Token::RangeFn,
            ..
        }
        | SpannedToken {
            token: Token::LastWord,
            ..
        }
        | SpannedToken {
            token: Token::Sqrt, ..
        }
        | SpannedToken {
            token: Token::Timer,
            ..
        }
        | SpannedToken {
            token: Token::Max, ..
        }
        | SpannedToken {
            token: Token::StrUpper,
            ..
        }
        | SpannedToken {
            token: Token::StrLower,
            ..
        }
        | SpannedToken {
            token: Token::Min, ..
        }
        | SpannedToken {
            token: Token::Zig, ..
        }
        | SpannedToken {
            token: Token::Mod, ..
        }
        | SpannedToken {
            token: Token::Trim, ..
        }
        | SpannedToken {
            token: Token::StrReverse,
            ..
        }
        | SpannedToken {
            token: Token::ConvertString,
            ..
        }
        | SpannedToken {
            token: Token::Round,
            ..
        }
        | SpannedToken {
            token: Token::Floor,
            ..
        }
        | SpannedToken {
            token: Token::Ceil, ..
        } => {
            let result = parse_builtin(tokens, &token)?;
            Ok(result)
        }
        SpannedToken {
            token: Token::Backtick,
            ..
        } => parse_template_string_expr(tokens),
        SpannedToken {
            token: Token::Identifier(s),
            ..
        } => parse_identifier(tokens, s),
        SpannedToken {
            token: Token::FunctionDef,
            ..
        } => parse_function_def(tokens),
        SpannedToken {
            token: Token::ListStart,
            span,
        } => literals_parse(
            SpannedToken {
                token: Token::ListStart,
                span,
            },
            tokens,
        ),
        SpannedToken {
            token: Token::LParen,
            ..
        } => {
            let pars = parse_expression(tokens)?;
            expect_token(tokens, Token::RParen)?;
            Ok(pars)
        }
        SpannedToken {
            token: Token::Loop, ..
        } => parse_loop(tokens),
        SpannedToken {
            token: Token::Subtract,
            ..
        } => {
            let expr = parse_single_expr(tokens)?;
            match expr {
                Expr::Number(i) => Ok(Expr::Number(-1 * i)),
                Expr::Float(f) => Ok(Expr::Float(-1.0 * f)),
                Expr::VariableRef { name, symbol } => Ok(Expr::BinaryOp {
                    left: Box::new(Expr::Number(-1)),
                    right: Box::new(Expr::VariableRef { name, symbol }),
                    op: crate::ast::Operation::Multiply,
                    return_type: Type::Integer,
                }),
                _ => return Err(ParserError::UnexpectedEOF),
            }
        }
        SpannedToken {
            token: Token::Not, ..
        } => {
            let expr = parse_single_expr(tokens)?;
            Ok(Expr::UnaryOp {
                op: Operation::Not,
                expr: Box::new(expr),
            })
        }

        other => Err(ParserError::UnexpectedToken(
            other.span.clone(),
            other.token.clone(),
        )),
    }
}

/*
Token::Type => parse_union_type(tokens),
Token::This => parse_identifier(tokens, "self"),
Token::Object => parse_struct_def(tokens),
Token::Enum => parse_enum_decl(tokens),
Token::Match => parse_match(tokens),
Token::Operator(op) if op == "-" => Ok(Expr::UnaryOp {
    op,
    expr: Box::new(parse_single_expr(tokens)?),
}),
,
Token::Conditional => parse_if_expr(tokens),

Token::Print
| Token::Input
| Token::Len
| Token::NumberFn
| Token::Sum
| Token::RangeFn
| Token::LastWord
| Token::Sqrt
| Token::Timer
| Token::Max
| Token::StrUpper
| Token::StrLower
| Token::Min
| Token::Zig
| Token::Mod
| Token::Trim
| Token::StrReverse
| Token::ConvertString
| Token::Round
| Token::Floor
| Token::Ceil => {
    let result = parse_builtin(tokens, token)?;
    Ok(result)
}
Token::Eof | Token::Semicolon | Token::Newline => Err(ParserError::UnexpectedEOF), */
/* pub fn parse_single_expr<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let token = match tokens.next() {
        Some(t) => t,
        None => Err(ParserError::UnexpectedEOF)?,
    };
    match token {
        Token::StringLiteral(_s) => literals_parse(token, tokens),
        Token::True => Ok(Expr::Bool(true)),
        Token::False => Ok(Expr::Bool(false)),
        Token::Break => Ok(Expr::Break),
        Token::Continue => Ok(Expr::Continue),

        Token::Float(_num) => literals_parse(&token, tokens),
        Token::Backtick => parse_template_string_expr(tokens),
        Token::Number(_num) => literals_parse(&token, tokens),
        Token::This => parse_identifier(tokens, "self"),
        Token::Object => parse_struct_def(tokens),
        Token::Enum => parse_enum_decl(tokens),
        Token::ListStart => literals_parse(token, tokens),
        Token::ConstantDecl => parse_decl(tokens, false),
        Token::MutableDecl => parse_decl(tokens, true),
        Token::Match => parse_match(tokens),
        Token::Return => {
            let returned_value = parse_expression(tokens)?;
            Ok(Expr::Return(Box::new(returned_value)))
        }
        Token::FunctionDef => parse_function_def(tokens),
        Token::Operator(op) if op == "-" => Ok(Expr::UnaryOp {
            op,
            expr: Box::new(parse_single_expr(tokens)?),
        }),
        Token::Comment(s) => Ok(Expr::Comment(s)),
        Token::Loop => parse_loop(tokens),
        Token::Identifier(s) => parse_identifier(tokens, s),
        Token::Type => parse_union_type(tokens),
        Token::Conditional => parse_if_expr(tokens),

        Token::Print
        | Token::Input
        | Token::Len
        | Token::NumberFn
        | Token::Sum
        | Token::RangeFn
        | Token::LastWord
        | Token::Sqrt
        | Token::Timer
        | Token::Max
        | Token::StrUpper
        | Token::StrLower
        | Token::Min
        | Token::Zig
        | Token::Mod
        | Token::Trim
        | Token::StrReverse
        | Token::ConvertString
        | Token::Round
        | Token::Floor
        | Token::Ceil => {
            let result = parse_builtin(tokens, token)?;
            Ok(result)
        }
        Token::Eof | Token::Semicolon | Token::Newline => Err(ParserError::UnexpectedEOF),
        other => Err(ParserError::UnexpectedToken(other.clone())),
    }
}
 */
