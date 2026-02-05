#[cfg(test)]
mod tests {
    use interpreter::interpreter;

    #[test]
    fn print_hello_world() {
        let mut output = String::new();
        interpreter("../examples/hello_world.az", &mut output).unwrap();

        assert_eq!(output, "Salam 1312 ");
    }
    #[test]
    fn print_variables() {
        let mut output = String::new();
        interpreter("../examples/variables.az", &mut output).unwrap();
        assert_eq!(output, "5yeni a dəyəri: 2 b dəyəri Salam");
    }
}
