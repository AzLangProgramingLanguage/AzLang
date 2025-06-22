use serde::Deserialize;
use serde_json::from_str;

#[derive(Deserialize)]
pub struct Syntax {
    pub print: String,
    pub mutable_decl: String,
    pub constant_decl: String,
    pub function_def: String,
    pub conditional: String,
    pub _else: String,
    pub else_if: String,
    pub drop: String,
    pub integer: String,
    pub biginteger: String,
    pub lowinteger: String,
    pub string: String,
    pub _loop: String,
    pub len: String,
    pub input: String,
    pub listtype: String,
    pub bool: String,
    pub return_name: String,
    pub true_str: String,
    pub false_str: String,
    pub in_str: String,
    pub break_str: String,
    pub continue_str: String,
    pub end_str: String,
    pub object_str: String,
    pub method_str: String,
    pub this_str: String,
    pub tipdecl_str: String,
    pub match_str: String,
    pub arrow_str: String,
}

impl Syntax {
    pub fn load() -> std::result::Result<Self, std::io::Error> {
        let contents = include_str!("./config.json");
        let syntax = from_str(&contents)?;
        Ok(syntax)
    }
    pub fn is_type_str(&self, word: &str) -> bool {
        word == self.string
            || word == self.integer
            || word == self.biginteger
            || word == self.lowinteger
    }
}
