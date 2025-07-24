use std::fmt;

use crate::{lexer::Token, parser::ast::BuiltInFunction};

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
            "ədəd" => Token::NaturalType,
            "tam" => Token::IntegerType,
            "mətn" => Token::StringType,
            "simvol" => Token::CharType,
            "böyük_ədəd" => Token::BigIntegerType,
            "kiçik_ədəd" => Token::LowIntegerType,
            "kəsr" => Token::FloatType,
            "qərar" => Token::BoolType,
            "Çap" => Token::Print,
            "Giriş" => Token::Input,
            "Maksimum" => Token::Max,
            "Minimum" => Token::Min,
            "VaxtAl" => Token::Timer,
            "Modul" => Token::Mod,
            "Ədəd" => Token::NumberFn,
            "Cəm" => Token::Sum,
            "Yuvarlaqlaşdır" => Token::Round,
            "AşağıYuvarlaqlaşdır" => Token::Floor,
            "YuxarıYuvarlaqlaşdır" => Token::Ceil,
            "Uzunluq" => Token::Len,
            "Sonsöz" => Token::LastWord,
            "Kök" => Token::Sqrt,
            "aralıq" => Token::RangeFn,
            "növ" => Token::Enum,
            "metod" => Token::Method,
            other => Token::Identifier(other.to_string()),
        },
    }
}

impl fmt::Display for BuiltInFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            BuiltInFunction::Print => "Çap",
            BuiltInFunction::Len => "Uzunluq",
            BuiltInFunction::Range => "Aralıq",
            BuiltInFunction::Timer => "Timer",
            BuiltInFunction::Input => "Giriş",
            BuiltInFunction::Sum => "Cəm",
            BuiltInFunction::Sqrt => "Kvadrat kök",
            BuiltInFunction::Round => "Yuvarlama",
            BuiltInFunction::Floor => "Yuvarlama",
            BuiltInFunction::Ceil => "Yuvarlama",
            BuiltInFunction::Mod => "Modul",
            BuiltInFunction::Max => "Maksimum",
            BuiltInFunction::Min => "Minimum",
            BuiltInFunction::Number => "Ədəd",
            BuiltInFunction::LastWord => "Sonsöz",
        };
        write!(f, "{name}")
    }
}
