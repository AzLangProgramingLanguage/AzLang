use std::iter::Peekable;
use std::str::Chars;

pub fn skip_whitespace(chars: &mut Peekable<Chars>) {
    while let Some(&ch) = chars.peek() {
        if ch == ' ' || ch == '\t' || ch == '\r' {
            chars.next();
        } else {
            break;
        }
    }
}
