struct SourceSpan {
    start: u32,
    end: u32,
    line: u32,
}
struct SpannedToken {
    token: String,
    span: SourceSpan,
}
struct Tokens {
    state: usize,
    source: Vec<SpannedToken>,
}

impl Iterator for Tokens {
    type Item = SpannedToken;
    fn next(&mut self) -> Option<SpannedToken> {
        if self.state < self.source.len() {
            return Some(self.source.pop()?);
        } else {
            return None;
        }
    }
}

fn main() {
    println!("Hello, world! , {}", 1);
}
