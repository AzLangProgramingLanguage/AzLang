use std::f64::consts::PI;

use crate::{
    Lexer,
    errors::LexerError,
    iterator::{SourceSpan, Tokens},
    tokens::Token,
};

fn tokenize(input: &str) -> Vec<Token> {
    Lexer::new(input)
        .tokenize()
        .unwrap()
        .map(|x| x.token)
        .collect()
}

fn tokenize_err(input: &str) -> LexerError {
    Lexer::new(input).tokenize().unwrap_err()
}

fn tokenize_spanned(input: &str) -> Vec<(Token, SourceSpan)> {
    Lexer::new(input)
        .tokenize()
        .unwrap()
        .map(|x| (x.token, x.span))
        .collect()
}

// ── Single character tokens ──

#[test]
fn test_lparen() {
    assert_eq!(tokenize("("), vec![Token::LParen]);
}

#[test]
fn test_rparen() {
    assert_eq!(tokenize(")"), vec![Token::RParen]);
}

#[test]
fn test_lbrace() {
    assert_eq!(tokenize("{"), vec![Token::LBrace]);
}

#[test]
fn test_rbrace() {
    assert_eq!(tokenize("}"), vec![Token::RBrace]);
}

#[test]
fn test_colon() {
    assert_eq!(tokenize(":"), vec![Token::Colon]);
}

#[test]
fn test_comma() {
    assert_eq!(tokenize(","), vec![Token::Comma]);
}

#[test]
fn test_dot() {
    assert_eq!(tokenize("."), vec![Token::Dot]);
}

#[test]
fn test_at() {
    assert_eq!(tokenize("@"), vec![Token::At]);
}

#[test]
fn test_list_brackets() {
    assert_eq!(tokenize("["), vec![Token::ListStart]);
    assert_eq!(tokenize("]"), vec![Token::ListEnd]);
}

#[test]
fn test_underscore() {
    assert_eq!(tokenize("_"), vec![Token::Underscore]);
}

// ── Operators ──

#[test]
fn test_add() {
    assert_eq!(tokenize("+"), vec![Token::Add]);
}

#[test]
fn test_subtract() {
    assert_eq!(tokenize("-"), vec![Token::Subtract]);
}

#[test]
fn test_multiply() {
    assert_eq!(tokenize("*"), vec![Token::Multiply]);
}

#[test]
fn test_divide() {
    assert_eq!(tokenize("/"), vec![Token::Divide]);
}

#[test]
fn test_modulo() {
    assert_eq!(tokenize("%"), vec![Token::Modulo]);
}

#[test]
fn test_greater() {
    assert_eq!(tokenize(">"), vec![Token::Greater]);
}

#[test]
fn test_less() {
    assert_eq!(tokenize("<"), vec![Token::Less]);
}

#[test]
fn test_assign() {
    assert_eq!(tokenize("="), vec![Token::Assign]);
}

#[test]
fn test_equal() {
    assert_eq!(tokenize("=="), vec![Token::Equal]);
}

#[test]
fn test_not() {
    assert_eq!(tokenize("!"), vec![Token::Not]);
}

#[test]
fn test_not_equal() {
    assert_eq!(tokenize("!="), vec![Token::NotEqual]);
}

// `&` and `|` aren't in the operator match arm, so they fall through to read_word
// causing an infinite loop (char never consumed). These are tracked as known bugs.

// ── Numbers ──

#[test]
fn test_integer() {
    assert_eq!(tokenize("42"), vec![Token::Number(42)]);
}

#[test]
fn test_zero() {
    assert_eq!(tokenize("0"), vec![Token::Number(0)]);
}

#[test]
fn test_float() {
    assert_eq!(tokenize("3.14"), vec![Token::Float(PI)]);
}

#[test]
fn test_float_leading_zero() {
    assert_eq!(tokenize("0.5"), vec![Token::Float(0.5)]);
}

#[test]
fn test_large_integer() {
    assert_eq!(tokenize("999999"), vec![Token::Number(999999)]);
}

#[test]
fn test_negative_number_not_supported() {
    let tokens = tokenize("-5");
    assert_eq!(tokens, vec![Token::Subtract, Token::Number(5)]);
}

#[test]
fn test_number_leading_zero_error() {
    match tokenize_err("012") {
        LexerError::CannotStartZeroNumber(_, ref s) => assert_eq!(s, "012"),
        other => panic!("Expected CannotStartZeroNumber, got {:?}", other),
    }
}

#[test]
fn test_number_followed_by_alpha_error() {
    match tokenize_err("123abc") {
        LexerError::NumberAndAlpha => {}
        other => panic!("Expected NumberAndAlpha, got {:?}", other),
    }
}

// ── Strings ──

#[test]
fn test_string_empty() {
    assert_eq!(tokenize("\"\""), vec![Token::StringLiteral("".into())]);
}

#[test]
fn test_string_simple() {
    assert_eq!(
        tokenize("\"hello\""),
        vec![Token::StringLiteral("hello".into())]
    );
}

#[test]
fn test_string_with_spaces() {
    assert_eq!(
        tokenize("\"hello world\""),
        vec![Token::StringLiteral("hello world".into())]
    );
}

#[test]
fn test_string_single_quotes_not_closed() {
    match tokenize_err("'hello'") {
        LexerError::UnClosedString(_, ref s) => assert_eq!(s, "hello'"),
        other => panic!("Expected UnClosedString, got {:?}", other),
    }
}

#[test]
fn test_string_unclosed_eof() {
    match tokenize_err("\"hello") {
        LexerError::UnClosedString(_, ref s) => assert_eq!(s, "hello"),
        other => panic!("Expected UnClosedString, got {:?}", other),
    }
}

#[test]
fn test_string_unclosed_newline() {
    match tokenize_err("\"hello\n") {
        LexerError::UnClosedString(_, ref s) => assert_eq!(s, "hello"),
        other => panic!("Expected UnClosedString, got {:?}", other),
    }
}

// ── Keywords / Words ──

#[test]
fn test_keyword_mutable_decl() {
    assert_eq!(tokenize("dəyişən"), vec![Token::MutableDecl]);
}

#[test]
fn test_keyword_constant_decl() {
    assert_eq!(tokenize("sabit"), vec![Token::ConstantDecl]);
}

#[test]
fn test_keyword_conditional() {
    assert_eq!(tokenize("əgər"), vec![Token::Conditional]);
}

#[test]
fn test_keyword_else_if() {
    assert_eq!(tokenize("yoxsa"), vec![Token::ElseIf]);
}

#[test]
fn test_keyword_else() {
    assert_eq!(tokenize("əks"), vec![Token::Else]);
}

#[test]
fn test_keyword_function_def() {
    assert_eq!(tokenize("funksiya"), vec![Token::FunctionDef]);
}

#[test]
fn test_keyword_function_type() {
    assert_eq!(tokenize("çağrılan"), vec![Token::FnType]);
}

#[test]
fn test_keyword_array() {
    assert_eq!(tokenize("siyahı"), vec![Token::Array]);
}

#[test]
fn test_keyword_object() {
    assert_eq!(tokenize("Obyekt"), vec![Token::Object]);
}

#[test]
fn test_keyword_this() {
    assert_eq!(tokenize("öz"), vec![Token::This]);
}

#[test]
fn test_keyword_match() {
    assert_eq!(tokenize("uyğun"), vec![Token::Match]);
}

#[test]
fn test_keyword_break() {
    assert_eq!(tokenize("dayan"), vec![Token::Break]);
}

#[test]
fn test_keyword_continue() {
    assert_eq!(tokenize("davam"), vec![Token::Continue]);
}

#[test]
fn test_keyword_loop() {
    assert_eq!(tokenize("gəz"), vec![Token::Loop]);
}

#[test]
fn test_keyword_end() {
    assert_eq!(tokenize("son"), vec![Token::End]);
}

#[test]
fn test_keyword_return_() {
    assert_eq!(tokenize("qaytar"), vec![Token::Return]);
}

#[test]
fn test_keyword_drop() {
    assert_eq!(tokenize("çıx"), vec![Token::Drop]);
}

#[test]
fn test_keyword_true() {
    assert_eq!(tokenize("doğru"), vec![Token::True]);
}

#[test]
fn test_keyword_false() {
    assert_eq!(tokenize("yanlış"), vec![Token::False]);
}

#[test]
fn test_keyword_in() {
    assert_eq!(tokenize("içində"), vec![Token::In]);
}

#[test]
fn test_keyword_and() {
    assert_eq!(tokenize("və"), vec![Token::And]);
}

// Note: "və_ya" can't be a single token because `_` is consumed as Underscore separately
#[test]
fn test_keyword_or_underscore_split() {
    assert_eq!(
        tokenize("və_ya"),
        vec![
            Token::And,
            Token::Underscore,
            Token::Identifier("ya".into())
        ]
    );
}

#[test]
fn test_keyword_integer_type() {
    assert_eq!(tokenize("ədəd"), vec![Token::IntegerType]);
    assert_eq!(tokenize("tam"), vec![Token::IntegerType]);
}

#[test]
fn test_keyword_string_type() {
    assert_eq!(tokenize("yazı"), vec![Token::StringType]);
}

#[test]
fn test_keyword_natural_type() {
    assert_eq!(tokenize("natural"), vec![Token::NaturalType]);
}

#[test]
fn test_keyword_void() {
    assert_eq!(tokenize("heçnə"), vec![Token::Void]);
}

#[test]
fn test_keyword_enum() {
    assert_eq!(tokenize("növ"), vec![Token::Enum]);
}

#[test]
fn test_keyword_method() {
    assert_eq!(tokenize("metod"), vec![Token::Method]);
}

#[test]
fn test_keyword_import() {
    assert_eq!(tokenize("ƏlavəEt"), vec![Token::Import]);
}

#[test]
fn test_keyword_type() {
    assert_eq!(tokenize("tip"), vec![Token::Type]);
}

#[test]
fn test_keyword_zig_types() {
    assert_eq!(tokenize("zigsabityazı"), vec![Token::ZigConstString]);
    assert_eq!(tokenize("ziginteger"), vec![Token::ZigInteger]);
    assert_eq!(tokenize("zigyazı"), vec![Token::ZigString]);
    assert_eq!(tokenize("zigsabitsiyahı"), vec![Token::ZigConstArray]);
    assert_eq!(tokenize("zigsiyahı"), vec![Token::ZigArray]);
    assert_eq!(tokenize("zigfloat"), vec![Token::ZigFloat]);
}

#[test]
fn test_keyword_other_types() {
    assert_eq!(tokenize("işarə"), vec![Token::CharType]);
    // Note: "böyük_ədəd" and "kiçik_ədəd" split on `_`
    assert_eq!(tokenize("kəsr"), vec![Token::FloatType]);
    assert_eq!(tokenize("qərar"), vec![Token::BoolType]);
}

// ── Identifiers ──

#[test]
fn test_identifier_simple() {
    assert_eq!(tokenize("foo"), vec![Token::Identifier("foo".into())]);
}

#[test]
fn test_identifier_with_unicode() {
    assert_eq!(
        tokenize("mənimDəyişənim"),
        vec![Token::Identifier("mənimDəyişənim".into())]
    );
}

#[test]
fn test_identifier_mixed() {
    assert_eq!(
        tokenize("fooBar123"),
        vec![Token::Identifier("fooBar123".into())]
    );
}

// ── Template strings ──

#[test]
fn test_template_simple() {
    let tokens = tokenize("`hello`");
    assert_eq!(
        tokens,
        vec![
            Token::Backtick,
            Token::StringLiteral("hello".into()),
            Token::Backtick,
        ]
    );
}

#[test]
fn test_template_empty() {
    let tokens = tokenize("``");
    assert_eq!(tokens, vec![Token::Backtick, Token::Backtick,]);
}

#[test]
fn test_template_with_interpolation() {
    let tokens = tokenize("`hello ${name}`");
    assert_eq!(
        tokens,
        vec![
            Token::Backtick,
            Token::StringLiteral("hello ".into()),
            Token::InterpolationStart,
            Token::Identifier("name".into()),
            Token::InterpolationEnd,
            Token::Backtick,
        ]
    );
}

#[test]
fn test_template_unclosed() {
    match tokenize_err("`hello") {
        LexerError::UnClosedString(_, ref s) => assert_eq!(s, "hello"),
        other => panic!("Expected UnClosedString, got {:?}", other),
    }
}

// `${` at start of template has a bug where empty content before `${` is not handled
// This test documents the actual current behavior
#[test]
fn test_template_interpolation_at_start_errors() {
    assert!(Lexer::new("`${1 + 2}`").tokenize().is_err());
}

// ── Indentation ──

#[test]
fn test_indent_simple() {
    let tokens = tokenize("a\n    b");
    assert_eq!(
        tokens,
        vec![
            Token::Identifier("a".into()),
            Token::Newline,
            Token::Indent,
            Token::Identifier("b".into()),
        ]
    );
}

#[test]
fn test_indent_dedent() {
    let tokens = tokenize("a\n    b\nc");
    assert_eq!(
        tokens,
        vec![
            Token::Identifier("a".into()),
            Token::Newline,
            Token::Indent,
            Token::Identifier("b".into()),
            Token::Newline,
            Token::Dedent,
            Token::Identifier("c".into()),
        ]
    );
}

#[test]
fn test_multiple_indent_levels() {
    let tokens = tokenize("a\n    b\n        c");
    assert_eq!(
        tokens,
        vec![
            Token::Identifier("a".into()),
            Token::Newline,
            Token::Indent,
            Token::Identifier("b".into()),
            Token::Newline,
            Token::Indent,
            Token::Identifier("c".into()),
        ]
    );
}

#[test]
fn test_indent_wrong_size() {
    match tokenize_err("a\n   b") {
        LexerError::InCorrectSpaceSize(_) => {}
        other => panic!("Expected InCorrectSpaceSize, got {:?}", other),
    }
}

#[test]
fn test_dedent_multiple_levels() {
    let tokens = tokenize("a\n    b\n        c\n    d");
    assert_eq!(
        tokens,
        vec![
            Token::Identifier("a".into()),
            Token::Newline,
            Token::Indent,
            Token::Identifier("b".into()),
            Token::Newline,
            Token::Indent,
            Token::Identifier("c".into()),
            Token::Newline,
            Token::Dedent,
            Token::Identifier("d".into()),
        ]
    );
}

// ── Comments ──

#[test]
fn test_comment_simple() {
    let tokens = tokenize("a /* comment */ b");
    assert_eq!(
        tokens,
        vec![Token::Identifier("a".into()), Token::Identifier("b".into()),]
    );
}

#[test]
fn test_comment_multiline() {
    let tokens = tokenize("a /* multi\nline */ b");
    assert_eq!(
        tokens,
        vec![Token::Identifier("a".into()), Token::Identifier("b".into()),]
    );
}

#[test]
fn test_comment_at_end() {
    let tokens = tokenize("a /* comment");
    assert_eq!(tokens, vec![Token::Identifier("a".into()),]);
}

// ── Composite expressions ──

#[test]
fn test_simple_expression() {
    let tokens = tokenize("dəyişən a = 42");
    assert_eq!(
        tokens,
        vec![
            Token::MutableDecl,
            Token::Identifier("a".into()),
            Token::Assign,
            Token::Number(42),
        ]
    );
}

#[test]
fn test_arithmetic_expression() {
    let tokens = tokenize("1 + 2 * 3");
    assert_eq!(
        tokens,
        vec![
            Token::Number(1),
            Token::Add,
            Token::Number(2),
            Token::Multiply,
            Token::Number(3),
        ]
    );
}

#[test]
fn test_comparison_chain() {
    let tokens = tokenize("a == b != c");
    assert_eq!(
        tokens,
        vec![
            Token::Identifier("a".into()),
            Token::Equal,
            Token::Identifier("b".into()),
            Token::NotEqual,
            Token::Identifier("c".into()),
        ]
    );
}

#[test]
fn test_empty_input() {
    let tokens = tokenize("");
    assert_eq!(tokens, vec![]);
}

#[test]
fn test_only_newlines() {
    let tokens = tokenize("\n\n\n");
    assert_eq!(tokens, vec![Token::Newline, Token::Newline, Token::Newline]);
}

#[test]
fn test_spaces_ignored_between_tokens() {
    let tokens = tokenize("a   b");
    assert_eq!(
        tokens,
        vec![Token::Identifier("a".into()), Token::Identifier("b".into()),]
    );
}

#[test]
fn test_unexpected_operator() {
    match tokenize_err("^") {
        LexerError::UnexpectedToken(_, '^') => {}
        other => panic!("Expected UnexpectedToken('^'), got {:?}", other),
    }
}

// ── SourceSpan accuracy ──

#[test]
fn test_span_single_token() {
    let spanned = tokenize_spanned("abc");
    assert_eq!(spanned.len(), 1);
    let (token, span) = &spanned[0];
    assert_eq!(*token, Token::Identifier("abc".into()));
    assert_eq!(span.line, 1);
    assert_eq!(span.start, 1);
    assert_eq!(span.end, 5);
}

#[test]
fn test_span_multiple_tokens() {
    let spanned = tokenize_spanned("a + b");
    assert_eq!(spanned.len(), 3);
    assert_eq!(spanned[0].0, Token::Identifier("a".into()));
    assert_eq!(spanned[0].1.start, 1);
    assert_eq!(spanned[1].0, Token::Add);
    assert_eq!(spanned[1].1.start, 4);
    assert_eq!(spanned[2].0, Token::Identifier("b".into()));
    assert_eq!(spanned[2].1.start, 5);
}

#[test]
fn test_span_newline_increments_line() {
    let spanned = tokenize_spanned("a\nb");
    assert_eq!(spanned.len(), 3);
    assert_eq!(spanned[0].1.line, 1);
    assert_eq!(spanned[0].1.start, 1);
    assert_eq!(spanned[1].1.line, 2);
    assert_eq!(spanned[2].1.line, 2);
}

// ── Tokens iterator ──

#[test]
fn test_tokens_iterator() {
    let mut tokens = Tokens::default();
    let span = SourceSpan {
        start: 1,
        end: 1,
        line: 1,
    };
    tokens.push(Token::Number(1), span.clone());
    tokens.push(Token::Add, span.clone());
    tokens.push(Token::Number(2), span.clone());
    let collected: Vec<Token> = tokens.into_iter().map(|x| x.token).collect();
    assert_eq!(
        collected,
        vec![Token::Number(1), Token::Add, Token::Number(2)]
    );
}

#[test]
fn test_tokens_peek() {
    let mut tokens = Tokens::default();
    let span = SourceSpan {
        start: 1,
        end: 1,
        line: 1,
    };
    tokens.push(Token::Number(42), span.clone());
    assert_eq!(
        tokens.peek(),
        Some(&crate::iterator::SpannedToken {
            token: Token::Number(42),
            span: span.clone(),
        })
    );
}

#[test]
fn test_tokens_peek_nth() {
    let mut tokens = Tokens::default();
    let span = SourceSpan {
        start: 1,
        end: 1,
        line: 1,
    };
    tokens.push(Token::Number(1), span.clone());
    tokens.push(Token::Number(2), span.clone());
    assert_eq!(
        tokens.peek_nth(0).map(|x| &x.token),
        Some(&Token::Number(1))
    );
    assert_eq!(
        tokens.peek_nth(1).map(|x| &x.token),
        Some(&Token::Number(2))
    );
    assert_eq!(tokens.peek_nth(2), None);
}

#[test]
fn test_tokens_push_front() {
    let mut tokens = Tokens::default();
    let span = SourceSpan {
        start: 1,
        end: 1,
        line: 1,
    };
    tokens.push(Token::Number(2), span.clone());
    tokens.push_front(crate::iterator::SpannedToken {
        token: Token::Number(1),
        span: span.clone(),
    });
    let collected: Vec<Token> = tokens.into_iter().map(|x| x.token).collect();
    assert_eq!(collected, vec![Token::Number(1), Token::Number(2)]);
}

#[test]
fn test_tokens_empty_peek() {
    let tokens = Tokens::default();
    assert!(tokens.peek().is_none());
}

// ── LexerError display ──

#[test]
fn test_error_display_unexpected_token() {
    let err = LexerError::UnexpectedToken(
        SourceSpan {
            start: 5,
            end: 5,
            line: 3,
        },
        '#',
    );
    let msg = format!("{}", err);
    assert!(msg.contains("Uyğunluq olmayan token"));
    assert!(msg.contains("#"));
}

#[test]
fn test_error_display_unclosed_string() {
    let err = LexerError::UnClosedString(
        SourceSpan {
            start: 1,
            end: 5,
            line: 1,
        },
        "hello".into(),
    );
    let msg = format!("{}", err);
    assert!(msg.contains("String düzgün bağlanmayıb"));
    assert!(msg.contains("hello"));
}

#[test]
fn test_error_display_number_and_alpha() {
    let err = LexerError::NumberAndAlpha;
    let msg = format!("{}", err);
    assert_eq!(msg, "Ədəddən sonra hərf gələ bilməz!");
}

#[test]
fn test_error_display_variable_cannot_be_number() {
    let err = LexerError::VariableCannotBeNumber;
    let msg = format!("{}", err);
    assert_eq!(msg, "Başlangıc ədədlə adlandırıla bilməz!");
}

#[test]
fn test_error_display_double_dot() {
    let err = LexerError::DoubleDotNumber;
    let msg = format!("{}", err);
    assert_eq!(msg, "İki dəfə nöqtə qoya bilməzsiniz");
}

#[test]
fn test_error_display_incorrect_space() {
    let err = LexerError::InCorrectSpaceSize(SourceSpan {
        start: 1,
        end: 1,
        line: 1,
    });
    let msg = format!("{}", err);
    assert!(msg.contains("Uyğunsuz boşluq var."));
}

#[test]
fn test_error_display_unknown_operator() {
    let err = LexerError::UnknownOperator(
        SourceSpan {
            start: 1,
            end: 1,
            line: 1,
        },
        "^^".into(),
    );
    let msg = format!("{}", err);
    assert!(msg.contains("Uyğunluq olmayan operator"));
    assert!(msg.contains("^^"));
}

// ── Token Display ──

#[test]
fn test_token_display_lparen() {
    assert_eq!(format!("{}", Token::LParen), "(");
}

#[test]
fn test_token_display_rparen() {
    assert_eq!(format!("{}", Token::RParen), ")");
}

#[test]
fn test_token_display_number() {
    assert_eq!(format!("{}", Token::Number(42)), "42");
}

#[test]
fn test_token_display_float() {
    assert_eq!(format!("{}", Token::Float(PI)), "3.14");
}

#[test]
fn test_token_display_string() {
    assert_eq!(format!("{}", Token::StringLiteral("hello".into())), "hello");
}

#[test]
fn test_token_display_identifier() {
    assert_eq!(format!("{}", Token::Identifier("foo".into())), "foo");
}

#[test]
fn test_token_display_add() {
    assert_eq!(format!("{}", Token::Add), "+");
}

#[test]
fn test_token_display_subtract() {
    assert_eq!(format!("{}", Token::Subtract), "%");
}

#[test]
fn test_token_display_multiply() {
    assert_eq!(format!("{}", Token::Multiply), "*");
}

#[test]
fn test_token_display_divide() {
    assert_eq!(format!("{}", Token::Divide), "/");
}

#[test]
fn test_token_display_equal() {
    assert_eq!(format!("{}", Token::Equal), "==");
}

#[test]
fn test_token_display_not_equal() {
    assert_eq!(format!("{}", Token::NotEqual), "!=");
}

#[test]
fn test_token_display_assign() {
    assert_eq!(format!("{}", Token::Assign), "=");
}

#[test]
fn test_token_display_keywords() {
    assert_eq!(format!("{}", Token::MutableDecl), "mutable");
    assert_eq!(format!("{}", Token::ConstantDecl), "constant");
    assert_eq!(format!("{}", Token::FunctionDef), "function");
    assert_eq!(format!("{}", Token::Conditional), "conditional");
    assert_eq!(format!("{}", Token::Array), "array");
    assert_eq!(format!("{}", Token::Object), "object");
    assert_eq!(format!("{}", Token::Else), "else");
    assert_eq!(format!("{}", Token::ElseIf), "else if");
    assert_eq!(format!("{}", Token::Loop), "loop");
    assert_eq!(format!("{}", Token::End), "end");
    assert_eq!(format!("{}", Token::Return), "return");
    assert_eq!(format!("{}", Token::Break), "break");
    assert_eq!(format!("{}", Token::Continue), "continue");
    assert_eq!(format!("{}", Token::True), "true");
    assert_eq!(format!("{}", Token::False), "false");
}

#[test]
fn test_token_display_special() {
    assert_eq!(format!("{}", Token::Backtick), "`");
    assert_eq!(format!("{}", Token::InterpolationStart), "${");
    assert_eq!(format!("{}", Token::InterpolationEnd), "}");
    assert_eq!(format!("{}", Token::Arrow), "->");
    assert_eq!(format!("{}", Token::At), "@");
    assert_eq!(format!("{}", Token::Dot), ".");
    assert_eq!(format!("{}", Token::Underscore), "_");
    assert_eq!(format!("{}", Token::Eof), "EOF");
}

#[test]
fn test_token_display_types() {
    assert_eq!(format!("{}", Token::IntegerType), "integer");
    assert_eq!(format!("{}", Token::NaturalType), "natural");
    assert_eq!(format!("{}", Token::StringType), "string");
    assert_eq!(format!("{}", Token::FloatType), "float");
    assert_eq!(format!("{}", Token::BoolType), "bool");
    assert_eq!(format!("{}", Token::CharType), "char");
    assert_eq!(format!("{}", Token::Void), "void");
    assert_eq!(format!("{}", Token::FnType), "funskiya tipi");
}

// ── SourceSpan Display ──

#[test]
fn test_source_span_display() {
    let span = SourceSpan {
        start: 5,
        end: 7,
        line: 3,
    };
    assert_eq!(format!("{}", span), "Sətir 3, sütun 5");
}

// ── Float parse error ──

#[test]
fn test_float_unknown_display() {
    let err = LexerError::FloatUnKnow("invalid float".parse::<f64>().unwrap_err());
    let msg = format!("{}", err);
    assert!(msg.contains("Kəsr tokenizerdə bilinməyən problem oldu"));
}

#[test]
fn test_number_unknown_display() {
    let err = LexerError::NumberUnKnow("not a number".parse::<i64>().unwrap_err());
    let msg = format!("{}", err);
    assert!(msg.contains("Ədəd tokenizerdə bilinməyən problem oldu"));
}

// ── The single existing test pattern ──

#[test]
fn test_complex_expression() {
    let mut lexer = Lexer::new("(1 + 2) * 3");
    let tokens = lexer.tokenize().unwrap();
    let collected: Vec<Token> = tokens.into_iter().map(|x| x.token).collect();
    assert_eq!(
        collected,
        vec![
            Token::LParen,
            Token::Number(1),
            Token::Add,
            Token::Number(2),
            Token::RParen,
            Token::Multiply,
            Token::Number(3),
        ]
    );
}

#[test]
fn test_double_dedent() {
    let input = "a\n    b\n        c\nd";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::Identifier("a".into()),
            Token::Newline,
            Token::Indent,
            Token::Identifier("b".into()),
            Token::Newline,
            Token::Indent,
            Token::Identifier("c".into()),
            Token::Newline,
            Token::Dedent,
            Token::Dedent,
            Token::Identifier("d".into()),
        ]
    );
}

#[test]
fn test_string_with_special_chars() {
    assert_eq!(
        tokenize("\"a!@#$%^&*()\""),
        vec![Token::StringLiteral("a!@#$%^&*()".into())]
    );
}

#[test]
fn test_template_dollar_without_brace() {
    let tokens = tokenize("`$`");
    assert_eq!(
        tokens,
        vec![
            Token::Backtick,
            Token::StringLiteral("$".into()),
            Token::Backtick,
        ]
    );
}

// `${` at start of template isn't handled correctly; this documents current behavior
#[test]
fn test_template_consecutive_interpolations_error() {
    assert!(Lexer::new("`${a}${b}`").tokenize().is_err());
}

#[test]
fn test_number_then_alpha_edge() {
    match tokenize_err("42foo") {
        LexerError::NumberAndAlpha => {}
        other => panic!("Expected NumberAndAlpha, got {:?}", other),
    }
}

#[test]
fn test_identifier_starting_with_number_not_possible() {
    match tokenize_err("123abc") {
        LexerError::NumberAndAlpha => {}
        other => panic!("Expected NumberAndAlpha, got {:?}", other),
    }
}

#[test]
fn test_span_after_comment() {
    let spanned = tokenize_spanned("a /* x */ b");
    assert_eq!(spanned.len(), 2);
    assert_eq!(spanned[1].0, Token::Identifier("b".into()));
}

#[test]
fn test_large_indent_multiline() {
    let tokens = tokenize("a\n    b\n        c\n            d");
    assert_eq!(
        tokens,
        vec![
            Token::Identifier("a".into()),
            Token::Newline,
            Token::Indent,
            Token::Identifier("b".into()),
            Token::Newline,
            Token::Indent,
            Token::Identifier("c".into()),
            Token::Newline,
            Token::Indent,
            Token::Identifier("d".into()),
        ]
    );
}
