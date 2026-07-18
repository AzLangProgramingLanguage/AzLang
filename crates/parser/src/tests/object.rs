#[cfg(test)]
use crate::errors::ParserError;
use crate::{helpers::expect_token, tests::create_tokens};
use tokenizer::{iterator::SpannedToken, tokens::Token};
#[test]
pub fn create_an_object() {
    let mut tokens = create_tokens(vec![Token::Object, Token::Identifier(String::from("Adam"))]);

    assert!(expect_token(&mut tokens, Token::Object).is_ok());
    if let Some(token) = tokens.next() {}
}
//
// pub fn parse_struct_def<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>, ParserError>
// where
//     I: Iterator<Item = &'a Token>,
// {
//     let name = match tokens.next() {
//         Some(Token::Identifier(name)) => (*name).as_str(),
//         None => return Err(ParserError::UnexpectedEOF),
//         Some(other) => return Err(ParserError::StructNameNotFound(other.clone())),
//     };
//     expect_token(tokens, Token::Newline)?;
//
//     let mut fields = Vec::new();
//     let mut methods: Vec<MethodType<'a>> = Vec::new();
//
//     expect_token(tokens, Token::Indent)?;
//
//     while let Some(token) = tokens.peek() {
//         match token {
//             Token::Identifier(field_name) => {
//                 let field_name = (*field_name).as_str();
//                 tokens.next();
//
//                 expect_token(tokens, Token::Colon)?;
//                 let field_type = parse_type(tokens)?;
//                 if let Some(Token::Operator(s)) = tokens.next()
//                     && s == "="
//                 {
//                     let value = parse_expression(tokens)?;
//                     fields.push((field_name, field_type, Some(value)));
//                 } else {
//                     fields.push((field_name, field_type, None));
//                 }
//
//                 skip_newlines(tokens)?;
//             }
//             Token::Method => {
//                 let method_expr = parse_method(tokens)?;
//                 methods.push(MethodType {
//                     name: method_expr.0,
//                     params: method_expr.1,
//                     body: method_expr.2,
//                     return_type: method_expr.3,
//                 });
//
//                 skip_newlines(tokens)?;
//             }
//             Token::Dedent => {
//                 tokens.next();
//                 break;
//             }
//             Token::Eof => break,
//             other => {
//                 return Err(ParserError::StructNotExpected((*other).clone()));
//             }
//         }
//     }
//     Ok(Expr::StructDef {
//         name,
//         fields,
//         methods,
//     })
// }
