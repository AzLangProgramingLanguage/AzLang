use validator::ast::Program;

pub fn transpile_program(program: Program) -> String {
    format!(
        "
export function w $main() {{               
    @start 
        call $exit(w 0)
        ret
    }}
"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
