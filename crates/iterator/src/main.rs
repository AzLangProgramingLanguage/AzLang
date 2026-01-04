use std::collections::VecDeque;

use file_system;
use logging::error;
use tokenizer;
#[derive(Debug)]
struct SourceSpan {
    start: u32,
    end: u32,
    line: u32,
}
#[derive(Debug)]
struct SpannedToken {
    token: String,
    span: SourceSpan,
}
struct Tokens {
    source: VecDeque<SpannedToken>,
}

impl Iterator for Tokens {
    type Item = SpannedToken;
    fn next(&mut self) -> Option<Self::Item> {
        self.source.pop_front()
    }
}
impl Tokens {
    fn peek(&self) -> Option<&SpannedToken> {
        self.source.front()
    }
    fn peek_nth(&self, index: usize) -> Option<&SpannedToken> {
        self.source.get(index)
    }
}

fn main() {
    let tokens = VecDeque::from([
        SpannedToken {
            token: "Salam".to_string(),
            span: SourceSpan {
                start: 0,
                end: 0,
                line: 0,
            },
        },
        SpannedToken {
            token: "Salam1".to_string(),
            span: SourceSpan {
                start: 0,
                end: 0,
                line: 0,
            },
        },
    ]);
    let mut tokens = Tokens { source: tokens };
    println!("{:?}", tokens.peek());
    tokens.next();
    println!("{:?}", tokens.peek());
    let sdk = file_system::read_file("test.az").expect("Error var");
    let mut tokeniz = tokenizer::new_lexer::NewLexer::new(&sdk);
    let real_tokens = tokeniz.tokenize();
    match real_tokens {
        Ok(tokens) => println!("{:?}", tokens),
        Err(e) => error(&e.to_string()),
    }
}
