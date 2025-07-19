use crate::{
    lexer::Token,
    parser::{
        ast::Expr,
        helper::{expect_token, skip_newlines},
        method::parse_method,
        types::parse_type,
    },
};
use color_eyre::eyre::{Result, eyre};
use peekmore::PeekMoreIterator;

pub fn parse_struct_def<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    tokens.next();
    // Struktur adını al
    let name = match tokens.next() {
        Some(Token::Identifier(name)) => (*name).as_str(),
        other => return Err(eyre!("Struktur adı gözlənilirdi, tapıldı: {:?}", other)),
    };
    expect_token(tokens, Token::Newline)?;

    let mut fields = Vec::new();
    let mut methods = Vec::new();

    expect_token(tokens, Token::Indent)?;

    while let Some(token) = tokens.peek() {
        match token {
            Token::Identifier(field_name) => {
                let field_name = (*field_name).as_str();
                tokens.next();

                expect_token(tokens, Token::Colon)?;
                let field_type = parse_type(tokens)?;
                fields.push((field_name, field_type));

                skip_newlines(tokens)?;
            }
            Token::Method => {
                let method_expr = parse_method(tokens)?;
                methods.push(method_expr);

                skip_newlines(tokens)?;
            }
            Token::Dedent => {
                tokens.next();
                break;
            }
            Token::Eof => break,
            other => {
                return Err(eyre!("Struct daxilində gözlənilməz token: {:?}", other));
            }
        }
    }
    Ok(Expr::StructDef {
        name,
        fields,
        methods,
    })
}
