struct SourceSpan {
    start: u32,
    end: u32,
    line: u32,
}
struct SpannedToken {
    token: String,
    span: SourceSpan,
}
struct Tokens {
    state: usize,
    source: Vec<SpannedToken>,
}

impl Iterator for Tokens {
    type Item = SpannedToken;
    fn next(&mut self) -> Option<SpannedToken> {
        if self.state < self.source.len() {
            return Some(self.source.pop()?);
        } else {
            return None;
        }
    }
}

fn main() {
    println!("Hello, world! , {}", 1);
}
#[warn(unused_doc_comments)]
fn test() {
    /*
     *
     *   Necə varsa elədə runtimeye gedir.  Emeliyyat sayı çox
     *   Runtimeda necə yazılıbsa eləcədə görünür.
     * */
    let a = "Salam";
    if a == "Salam" {
        println!("Ok")
    } else if a == "Aleykume" {
        println!("No")
    } else if a == "Super" {
        println!("No")
    }

    /*
     *
     *  Runtimeda görünəm. a="Salam"; println("Ok");
     *
     * */
    match a {
        "Salam" => println!("Ok"),
        "Aleykume" => println!("No"),
        "Super" => println!("No"),
        _ => println!("No"),
    }
}
