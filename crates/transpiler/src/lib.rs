use validator::ast::{
    Ast::{self},
    Expr, Program,
};

pub fn expr_transpiler(stream: &mut String, staticstring: &mut Vec<String>, expr: Expr) {
    match expr {
        Expr::Call {
            target,
            name,
            args,
            returned_type,
        } => {
            stream.push_str(&format!("call ${name}("));

            for arg in args {
                expr_transpiler(stream, staticstring, arg);
            }
            stream.push(')');
        }
        Expr::Number(num) => {
            stream.push_str(&format!("w {num}"));
        }
        Expr::String(str) => {
            staticstring.push(str);
            stream.push_str(&format!("l $str{}", staticstring.len() - 1));
        }
        _ => {}
    }
}
pub fn transpile_program(program: Program) -> String {
    let mut exprstream = String::new();
    let mut staticstringdata: Vec<String> = vec![];
    for ast in program.expressions {
        match ast {
            Ast::Expr(expr) => expr_transpiler(&mut exprstream, &mut staticstringdata, expr),
            _ => {}
        }
    }
    let mut data = String::new();
    for (i, str) in staticstringdata.into_iter().enumerate() {
        data.push_str(&format!("data $str{i}= {{ b \"{str}\", b 0 }}\n"));
    }
    format!(
        "{data}
export function w $main() {{               
    @start 
        {exprstream}
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
