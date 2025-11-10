use errors::{Errors, ParserError};
pub fn compiler(path: &str) -> Result<(), impl Errors> {
    println!("Hello");
    Err(ParserError::UnexpectedToken("Salam".into()))
}
