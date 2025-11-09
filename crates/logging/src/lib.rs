pub fn parser_log(msg: &str) {
    println!("\x1b[36m[Böyük Qardaş Parserci]:\x1b[0m {}", msg);
}
pub fn error(msg: &str) {
    println!("\x1b[31m[Xəta]:\x1b[0m {}", msg);
    std::process::exit(1);
}
pub fn validator_log(msg: &str) {
    println!("\x1b[33m[Dəmir Əmi Validator]:\x1b[0m {}", msg);
}
pub fn translator_log(msg: &str) {
    println!("\x1b[34m[Kiçik bacı Tərcüməci]:\x1b[0m {}", msg);
}
