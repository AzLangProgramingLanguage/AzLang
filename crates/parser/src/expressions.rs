use std::collections::HashMap;

use crate::{
    ast::{Expr, Operation, Program},
    binary_op::{parse_expression, parse_statement},
    builtin::parse_builtin,
    errors::ParserError,
    function::parse_function_def,
    identifier::parse_identifier,
    literal_parse::literals_parse,
    shared_ast::Type,
    template::parse_template_string_expr,
};
use tokenizer::{
    iterator::{SpannedToken, Tokens},
    tokens::Token,
};

pub fn parse_expression_block<'a>(tokens: &mut Tokens) -> Result<Program, ParserError> {
    let mut ast = Program {
        functions: HashMap::new(),
        expressions: vec![],
    };

    while let Some(token) = tokens.peek() {
        match token {
            SpannedToken {
                token: Token::Newline,
                ..
            } => {
                tokens.next();
                continue;
            }

            SpannedToken {
                token: Token::FunctionDef,
                ..
            } => {
                let (name, function) = parse_function_def(tokens)?;
                ast.functions.insert(name, function);
            }

            SpannedToken {
                token: Token::StringLiteral(_),
                ..
            }
            | SpannedToken {
                token: Token::Number(_),
                ..
            }
            | SpannedToken {
                token: Token::Float(_),
                ..
            } => {
                return Err(ParserError::NotUserDirectValue);
            }
            SpannedToken {
                token: Token::Eof, ..
            } => {
                break;
            }
            _ => {
                let expr = parse_statement(tokens)?;
                ast.expressions.push(expr);
            }
        }
    }
    Ok(ast)
}

pub fn parse_single_expr<'a>(tokens: &mut Tokens) -> Result<Expr, ParserError> {
    let token = tokens.peek().ok_or(ParserError::UnexpectedEOF)?;
    match token {
        SpannedToken {
            token: Token::StringLiteral(_),
            ..
        } => literals_parse(tokens),
        SpannedToken {
            token: Token::Float(_num),
            ..
        } => literals_parse(tokens),
        SpannedToken {
            token: Token::Number(_num),
            ..
        } => literals_parse(tokens),
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
        } => Ok(Expr::Comment(s.clone())),
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
            let result = parse_builtin(tokens)?;
            Ok(result)
        }
        SpannedToken {
            token: Token::Backtick,
            ..
        } => parse_template_string_expr(tokens),
        SpannedToken {
            token: Token::Identifier(s),
            ..
        } => parse_identifier(tokens, s.to_string()),

        SpannedToken {
            token: Token::ListStart,
            ..
        } => literals_parse(tokens),

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

        other => {
            panic!("{other:#?}");
            return Err(ParserError::UnexpectedToken(
                other.span.clone(),
                other.token.clone(),
            ));
        }
    }
}
