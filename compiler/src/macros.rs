#[macro_export]
macro_rules! dd {
    ($value:expr) => {{
        use std::process::exit;

        println!("{:#?}", $value);
        exit(1);
    }};
}
