use crate::{
    lexer::Token,
    parser::ast::{BuiltInFunction, Type},
};

pub fn tokenize_word(word: &str) -> Token {
    match word.parse::<i64>() {
        Ok(n) => Token::Number(n),
        Err(_) => match word {
            "dəyişən" => Token::MutableDecl,
            "sabit" => Token::ConstantDecl,
            "əgər" => Token::Conditional,
            "yoxsa" => Token::ElseIf,
            "əks" => Token::Else,
            "funksiya" => Token::FunctionDef,
            "siyahı" => Token::Array,
            "Obyekt" => Token::Object,
            "öz" => Token::This,
            "uyğun" => Token::Match,
            "dayan" => Token::Break,
            "gəz" => Token::Loop,
            "son" => Token::End,
            "->" => Token::Arrow,
            "qaytar" => Token::Return,
            "çıx" => Token::Drop,
            "doğru" => Token::True,
            "yanlış" => Token::False,
            "içində" => Token::In,
            "və" => Token::And,
            "və_ya" => Token::Or,
            "&&" => Token::DoubleAnd,
            "||" => Token::DoubleOr,
            "ədəd" => Token::TypeName(Type::Integer),
            "mətn" => Token::TypeName(Type::Metn),
            "simvol" => Token::TypeName(Type::Char),
            "böyük_ədəd" => Token::TypeName(Type::BigInteger),
            "kiçik_ədəd" => Token::TypeName(Type::LowInteger),
            other => Token::Identifier(other.to_string()),
        },
    }
}

pub fn match_builtin(name: &str) -> Option<(BuiltInFunction, Type)> {
    match name {
        "çap" => Some((BuiltInFunction::Print, Type::Metn)),
        "giriş" => Some((BuiltInFunction::Input, Type::Metn)),
        "maksimum" => Some((BuiltInFunction::Max, Type::Integer)),
        "minimum" => Some((BuiltInFunction::Min, Type::Integer)),
        "vaxt_al" => Some((BuiltInFunction::Timer, Type::Integer)),
        "Ədəd" => Some((BuiltInFunction::Number, Type::Integer)),
        "cəm" => Some((BuiltInFunction::Sum, Type::Integer)),
        "uzunluq" => Some((BuiltInFunction::Len, Type::Integer)),
        "sonsöz" => Some((BuiltInFunction::LastWord, Type::Metn)),
        "aralıq" => Some((
            BuiltInFunction::Range,
            Type::Siyahi(Box::new(Type::Integer)),
        )),
        _ => None,
    }
}
