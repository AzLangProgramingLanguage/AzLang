pub fn parser_log(msg: &str) {
    println!("\x1b[36m[Big Brother]:\x1b[0m {}", msg);
}
pub fn error(msg: &str) {
    println!("\x1b[31m[Big Brother]:\x1b[0m {}", msg);
}
pub fn validator_log(msg: &str) {
    println!("\x1b[33m[Uncle Iron Validator]:\x1b[0m {}", msg);
}
pub fn translator_log(msg: &str) {
    println!("\x1b[34m[Little Sister Translator]:\x1b[0m {}", msg);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}