use std::{borrow::Cow, collections::HashMap};
pub mod validate;
use crate::parser::ast::{Expr, Parameter, Symbol, Type};
pub use validate::validate_expr;
pub mod helpers;

#[derive(Debug)]
pub struct FunctionInfo<'a> {
    pub name: Cow<'a, str>,
    pub return_type: Option<Type<'a>>,
    pub parameters: Vec<Parameter<'a>>,
}
pub struct ValidatorContext<'a> {
    pub scopes: Vec<HashMap<String, Symbol<'a>>>,
    pub functions: HashMap<Cow<'a, str>, FunctionInfo<'a>>,
    pub struct_defs: HashMap<
        String,
        (
            Vec<(&'a str, Type<'a>)>, // fields
            Vec<(Cow<'a, str>, Option<Type<'a>>)>,
        ),
    >,
    pub enum_defs: HashMap<Cow<'a, str>, Vec<Cow<'a, str>>>,

    pub current_function: Option<String>,
    pub current_return: Option<Box<Expr<'a>>>,
    pub current_struct: Option<String>,
}

impl<'a> Default for ValidatorContext<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ValidatorContext<'a> {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            functions: HashMap::new(),
            struct_defs: HashMap::new(),
            enum_defs: HashMap::new(),
            current_function: None,
            current_return: None,
            current_struct: None,
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn declare_variable(&mut self, name: String, symbol: Symbol<'a>) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, symbol);
        }
    }

    pub fn lookup_variable(&self, name: &str) -> Option<Symbol<'a>> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.get(name) {
                return Some(symbol.clone());
            }
        }
        None
    }

    pub fn declare_function(&mut self, func: FunctionInfo<'a>) {
        self.functions.insert(func.name.clone(), func);
    }
}
