use crate::translations::parser_errors::ParserError;
use color_eyre::eyre::{Result, eyre};
use peekmore::PeekMoreIterator;

use crate::{
    lexer::Token,
    parser::{
        ast::Expr,
        builtin::parse_builtin,
        decl::parse_decl,
        enums::parse_enum_decl,
        function_def::parse_function_def,
        helper::literals_parse,
        if_expr::{parse_else_expr, parse_else_if_expr, parse_if_expr},
        list::parse_list,
        loops::parse_loop,
        r#match::parse_match,
        object::parse_struct_def,
        op_expr::parse_binary_op_expr,
        parse_identifier::parse_identifier,
        template::parse_template_string_expr,
    },
};

use super::parse_union_type::parse_union_type;

pub fn parse_expression_block<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Vec<Expr<'a>>>
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
                return Err(eyre!(
                    "Bir başa mətn, rəqəm və ya kəsr ədəd istifadə edə bilməzsiniz"
                ));
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

pub fn parse_expression<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    parse_binary_op_expr(tokens, 0)
}

pub fn parse_single_expr<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let token = tokens.next().ok_or(ParserError::Eof)?;

    match token {
        Token::StringLiteral(_s) => literals_parse(token, tokens),
        Token::True => Ok(Expr::Bool(true)),
        Token::False => Ok(Expr::Bool(false)),

        Token::Float(_num) => literals_parse(token, tokens),
        Token::Backtick => parse_template_string_expr(tokens),
        Token::Number(_num) => literals_parse(token, tokens),
        Token::This => parse_identifier(tokens, "self"),
        Token::Object => parse_struct_def(tokens),
        Token::Enum => parse_enum_decl(tokens),
        Token::ListStart => Ok(parse_list(tokens)),
        Token::ConstantDecl => Ok(parse_decl(tokens, false).unwrap()),
        Token::MutableDecl => Ok(parse_decl(tokens, true).unwrap()),
        Token::Return => {
            let returned_value = parse_expression(tokens)?;
            Ok(Expr::Return(Box::new(returned_value)))
        }

        Token::Match => parse_match(tokens),
        Token::FunctionDef => parse_function_def(tokens),
        Token::Operator(op) if op == "-" => Ok(Expr::UnaryOp {
            op,
            expr: Box::new(parse_single_expr(tokens)?),
        }),

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
        other => Err(ParserError::UnexpectedToken(format!("{:?}", other))),
    }
}
