use crate::{errors::ParserError, typed_ast::TypedExpr};
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

use crate::{
    binary_op::parse_binary_op_expr,
    builtin::parse_builtin,
    condition::{parse_else_expr, parse_else_if_expr, parse_if_expr},
    decl::parse_decl,
    function::parse_function_def,
    helpers::literals_parse,
    identifier::parse_identifier,
    r#enum::parse_enum_decl,
    r#loop::parse_loop,
    r#match::parse_match,
    structs::parse_struct_def,
    template::parse_template_string_expr,
    typed_ast::TypedExpr,
    union::parse_union_type,
};

pub fn parse_typed_expression<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
) -> Result<Vec<TypedExpr<'a>>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let mut ast = Vec::new();
    while let Some(token) = tokens.peek() {
        match token {
            Token::Newline | Token::Semicolon | Token::Indent => {
                tokens.next();
                continue;
            }
            Token::Import => {
                tokens.nth(3);
            }
            Token::StringLiteral(_) | Token::Number(_) | Token::Float(_) => {
                return Err(ParserError::NotUserDirectValue);
            }
            Token::Eof => {
                tokens.next();
            }
            _ => {
                let expr = parse_expression(tokens)?;
                ast.push(expr);
                while matches!(tokens.peek(), Some(Token::Semicolon | Token::Newline)) {
                    tokens.next();
                }
            }
        }
    }
    Ok(ast)
}

pub fn parse_expression<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
) -> Result<TypedExpr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    Ok(TypedExpr::Break)
    // parse_binary_op_expr(tokens, 0)
}

pub fn parse_single_expr<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
) -> Result<TypedExpr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let token = match tokens.next() {
        Some(t) => t,
        None => Err(ParserError::UnexpectedEOF)?,
    };
    match token {
        Token::StringLiteral(_s) => literals_parse(token, tokens),
        Token::True => Ok(TypedExpr::Bool(true)),
        Token::False => Ok(TypedExpr::Bool(false)),
        Token::Break => Ok(TypedExpr::Break),
        Token::Continue => Ok(TypedExpr::Continue),

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
            Ok(TypedExpr::Return(Box::new(returned_value)))
        }
        Token::FunctionDef => parse_function_def(tokens),
        Token::Operator(op) if op == "-" => Ok(TypedExpr::UnaryOp {
            op,
            expr: Box::new(parse_single_expr(tokens)?),
        }),
        Token::Comment(s) => Ok(TypedExpr::Comment(s)),
        Token::Loop => parse_loop(tokens),
        Token::Identifier(s) => parse_identifier(tokens, s),
        Token::Type => parse_union_type(tokens),
        Token::Conditional => parse_if_expr(tokens),
        Token::ElseIf => parse_else_if_expr(tokens),
        Token::Else => parse_else_expr(tokens),

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
