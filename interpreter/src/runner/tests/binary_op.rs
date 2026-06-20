#[cfg(test)]

mod tests {
    use crate::Runner;
    use crate::runner::binary_op::binary_op_runner;
    use crate::runner::runner::Value;
    use parser::ast::Operation;
    use parser::shared_ast::Type;

    #[test]
    fn binary_op_add_integer() {
        let mut runner = Runner::new();
        let result = binary_op_runner(
            &mut runner,
            Value::Number(5),
            Value::Number(3),
            Operation::Add,
            Some(Type::Integer),
        );
        assert_eq!(result, Value::Number(8));
    }

    #[test]
    fn binary_op_subtract_integer() {
        let mut runner = Runner::new();
        let result = binary_op_runner(
            &mut runner,
            Value::Number(10),
            Value::Number(4),
            Operation::Subtract,
            Some(Type::Integer),
        );
        assert_eq!(result, Value::Number(6));
    }

    #[test]
    fn binary_op_multiply_integer() {
        let mut runner = Runner::new();
        let result = binary_op_runner(
            &mut runner,
            Value::Number(3),
            Value::Number(4),
            Operation::Multiply,
            Some(Type::Integer),
        );
        assert_eq!(result, Value::Number(12));
    }

    #[test]
    fn binary_op_divide_integer() {
        let mut runner = Runner::new();
        let result = binary_op_runner(
            &mut runner,
            Value::Number(10),
            Value::Number(3),
            Operation::Divide,
            Some(Type::Integer),
        );
        assert_eq!(result, Value::Number(3));
    }

    #[test]
    fn binary_op_modulo_integer() {
        let mut runner = Runner::new();
        let result = binary_op_runner(
            &mut runner,
            Value::Number(10),
            Value::Number(3),
            Operation::Modulo,
            Some(Type::Integer),
        );
        assert_eq!(result, Value::Number(1));
    }

    #[test]
    fn binary_op_equal_true() {
        let mut runner = Runner::new();
        let result = binary_op_runner(
            &mut runner,
            Value::Number(5),
            Value::Number(5),
            Operation::Equal,
            None,
        );
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn binary_op_equal_false() {
        let mut runner = Runner::new();
        let result = binary_op_runner(
            &mut runner,
            Value::Number(5),
            Value::Number(3),
            Operation::Equal,
            None,
        );
        assert_eq!(result, Value::Bool(false));
    }

    #[test]
    fn binary_op_not_equal_true() {
        let mut runner = Runner::new();
        let result = binary_op_runner(
            &mut runner,
            Value::Number(5),
            Value::Number(3),
            Operation::NotEqual,
            None,
        );
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn binary_op_not_equal_false() {
        let mut runner = Runner::new();
        let result = binary_op_runner(
            &mut runner,
            Value::Number(5),
            Value::Number(5),
            Operation::NotEqual,
            None,
        );
        assert_eq!(result, Value::Bool(false));
    }

    #[test]
    fn binary_op_equal_non_number() {
        let mut runner = Runner::new();
        let result = binary_op_runner(
            &mut runner,
            Value::Bool(true),
            Value::Bool(true),
            Operation::Equal,
            None,
        );
        assert_eq!(result, Value::Bool(false));
    }

    #[test]
    fn binary_op_not_equal_non_number() {
        let mut runner = Runner::new();
        let result = binary_op_runner(
            &mut runner,
            Value::Bool(true),
            Value::Bool(true),
            Operation::NotEqual,
            None,
        );
        assert_eq!(result, Value::Bool(false));
    }

    #[test]
    fn binary_op_default_case() {
        let mut runner = Runner::new();
        let result = binary_op_runner(
            &mut runner,
            Value::Number(1),
            Value::Number(2),
            Operation::And,
            None,
        );
        assert_eq!(result, Value::Bool(false));
    }

    #[test]
    fn binary_op_add_float() {
        let mut runner = Runner::new();
        let result = binary_op_runner(
            &mut runner,
            Value::Float(2.5),
            Value::Float(3.5),
            Operation::Add,
            None,
        );
        assert_eq!(result, Value::Float(6.0));
    }

    #[test]
    fn binary_op_subtract_float() {
        let mut runner = Runner::new();
        let result = binary_op_runner(
            &mut runner,
            Value::Float(5.0),
            Value::Float(2.0),
            Operation::Subtract,
            None,
        );
        assert_eq!(result, Value::Float(3.0));
    }

    #[test]
    fn binary_op_multiply_float() {
        let mut runner = Runner::new();
        let result = binary_op_runner(
            &mut runner,
            Value::Float(3.0),
            Value::Float(2.0),
            Operation::Multiply,
            None,
        );
        assert_eq!(result, Value::Float(6.0));
    }

    #[test]
    fn binary_op_divide_float() {
        let mut runner = Runner::new();
        let result = binary_op_runner(
            &mut runner,
            Value::Float(10.0),
            Value::Float(3.0),
            Operation::Divide,
            None,
        );
        assert_eq!(result, Value::Float(10.0 / 3.0));
    }

    #[test]
    fn binary_op_modulo_float() {
        let mut runner = Runner::new();
        let result = binary_op_runner(
            &mut runner,
            Value::Float(10.0),
            Value::Float(3.0),
            Operation::Modulo,
            None,
        );
        assert_eq!(result, Value::Float(10.0 % 3.0));
    }

    #[test]
    fn binary_op_greater() {
        let mut runner = Runner::new();
        let result = binary_op_runner(
            &mut runner,
            Value::Number(5),
            Value::Number(3),
            Operation::Greater,
            None,
        );
        assert_eq!(result, Value::Bool(false));
    }

    #[test]
    fn binary_op_add_string() {
        let mut runner = Runner::new();
        let result = binary_op_runner(
            &mut runner,
            Value::String("Hello".to_string()),
            Value::String("World".to_string()),
            Operation::Add,
            Some(Type::String(parser::shared_ast::StringEnum::DynamicString)),
        );
        assert_eq!(result, Value::String("HelloWorld".to_string()));
    }

    #[test]
    fn binary_op_add_string_with_space() {
        let mut runner = Runner::new();
        let result = binary_op_runner(
            &mut runner,
            Value::String("Hello ".to_string()),
            Value::String("World".to_string()),
            Operation::Add,
            Some(Type::String(parser::shared_ast::StringEnum::DynamicString)),
        );
        assert_eq!(result, Value::String("Hello World".to_string()));
    }

    #[test]
    fn binary_op_add_empty_string() {
        let mut runner = Runner::new();
        let result = binary_op_runner(
            &mut runner,
            Value::String("".to_string()),
            Value::String("test".to_string()),
            Operation::Add,
            Some(Type::String(parser::shared_ast::StringEnum::DynamicString)),
        );
        assert_eq!(result, Value::String("test".to_string()));
    }
}
