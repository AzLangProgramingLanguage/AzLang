use tokenizer::{
    iterator::{SpannedToken, Tokens},
    tokens::Token,
};

use crate::{
    ast::Expr, errors::ParserError, expressions::parse_single_expr, list::parse_list,
    shared_ast::Type,
};

pub fn literals_parse<'a>(tokens: &mut Tokens) -> Result<Expr, ParserError> {
    let token = tokens.next().ok_or(ParserError::UnexpectedEOF)?;

    match &token.token {
        Token::StringLiteral(s) => return Ok(Expr::String(s.to_string())),
        Token::Number(num) => return Ok(Expr::Number(*num)),
        Token::Float(num) => return Ok(Expr::Float(*num)),
        Token::ListStart => return parse_list(tokens),
        _ => {
            return Err(ParserError::UnexpectedToken(
                token.span.clone(),
                token.token.clone(),
            ));
        }
    };

    /*     while let Some(Token::Dot) = tokens.peek() {
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
                               let arg = parse_single_expr(tokens)?;
                               args.push(arg);
                           }
                       }
                   }

                   expr = Expr::Call {
                       target: Some(Box::new(expr)),
                       name: field_or_method,
                       args,
                       returned_type: None,
                   };
               }
               _ => {
                   expr = Expr::Index {
                       target: Box::new(expr),
                       index: Box::new(Expr::String(field_or_method)),
                       target_type: Type::Any,
                   };
               }
           }
       }
    */
}
