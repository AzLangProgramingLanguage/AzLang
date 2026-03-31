use crate::runner::runner::Value;

pub fn input() -> Value {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Input girilmedi");
    Value::String(input)
}
