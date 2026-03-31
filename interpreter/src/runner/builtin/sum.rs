use crate::runner::runner::Value;

pub fn sum(args: Vec<Value>) -> Value {
    let mut result = Value::Number(0);
    for arg in args {
        match arg {
            Value::Number(n) => {
                result = match result {
                    Value::Number(c) => Value::Number(n + c),
                    Value::Float(c) => Value::Float(n as f64 + c),
                    _ => result,
                }
            }
            Value::Float(n) => {
                result = match result {
                    Value::Number(c) => Value::Float(n + c as f64),
                    Value::Float(c) => Value::Float(n + c),
                    _ => result,
                }
            }
            _ => {}
        }
    }
    result
}
