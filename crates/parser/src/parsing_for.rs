use crate::{
    binary_op_typed::parse_binary_op_expr_typed,
    builtin_typed::parse_builtin_typed,
    condition::{
        parse_else_expr, parse_else_expr_typed, parse_else_if_expr, parse_else_if_expr_typed,
        parse_if_expr, parse_if_expr_typed,
    },
    decl::{parse_decl, parse_decl_typed},
    r#enum::{parse_enum_decl, parse_enum_decl_typed},
    function::{parse_function_def, parse_function_def_typed},
    identifier::{parse_identifier, parse_identifier_typed},
    literal_parse_typed::literals_parse_typed,
    r#loop::{parse_loop, parse_loop_typed},
    r#match::{parse_match, parse_match_typed},
    structs::parse_struct_def_typed,
    template::parse_template_string_expr_typed,
    union::{parse_union_type, parse_union_type_typed},
};
use crate::{errors::ParserError, typed_ast::TypedExpr};
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

pub fn parse_expression_for_typed_ast<'a, I>(
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
                let expr = parse_expression_typed(tokens)?;
                ast.push(expr);
                while matches!(tokens.peek(), Some(Token::Semicolon | Token::Newline)) {
                    tokens.next();
                }
            }
        }
    }
    Ok(ast)
}

pub fn parse_expression_typed<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
) -> Result<TypedExpr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    parse_binary_op_expr_typed(tokens, 0)
}

pub fn parse_single_expr_typed<'a, I>(
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
        Token::StringLiteral(_s) => literals_parse_typed(token, tokens),
        Token::True => Ok(TypedExpr::Bool(true)),
        Token::False => Ok(TypedExpr::Bool(false)),
        Token::Break => Ok(TypedExpr::Break),
        Token::Continue => Ok(TypedExpr::Continue),

        Token::Float(_num) => literals_parse_typed(&token, tokens),
        Token::Backtick => parse_template_string_expr_typed(tokens),
        Token::Number(_num) => literals_parse_typed(&token, tokens),
        Token::This => parse_identifier_typed(tokens, "self"),
        Token::Object => parse_struct_def_typed(tokens),
        Token::Enum => parse_enum_decl_typed(tokens),
        Token::ListStart => literals_parse_typed(token, tokens),
        Token::ConstantDecl => parse_decl_typed(tokens, false),
        Token::MutableDecl => parse_decl_typed(tokens, true),
        Token::Match => parse_match_typed(tokens),
        Token::Return => {
            let returned_value = parse_expression_typed(tokens)?;
            Ok(TypedExpr::Return(Box::new(returned_value)))
        }
        Token::FunctionDef => parse_function_def_typed(tokens),
        Token::Operator(op) if op == "-" => Ok(TypedExpr::UnaryOp {
            op,
            expr: Box::new(parse_single_expr_typed(tokens)?),
        }),
        Token::Comment(s) => Ok(TypedExpr::Comment(s)),
        Token::Loop => parse_loop_typed(tokens),
        Token::Identifier(s) => parse_identifier_typed(tokens, s),
        Token::Type => parse_union_type_typed(tokens),
        Token::Conditional => parse_if_expr_typed(tokens),
        Token::ElseIf => parse_else_if_expr_typed(tokens),
        Token::Else => parse_else_expr_typed(tokens),

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
            let result = parse_builtin_typed(tokens, token)?;
            Ok(result)
        }
        Token::Eof | Token::Semicolon | Token::Newline => Err(ParserError::UnexpectedEOF),
        other => Err(ParserError::UnexpectedToken(other.clone())),
    }
}
