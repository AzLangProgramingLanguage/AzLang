pub mod helper;
pub mod validate;
use std::collections::HashMap;

pub use validate::validate_expr;

use crate::parser::{Expr, ast::Type};

#[derive(Clone, Debug)]
pub struct Location {
    pub line: usize,
    pub column: usize,
    pub file: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Parameter {
    pub name: String,
    pub typ: Type,
    pub is_mutable: bool,
    pub is_pointer: bool,
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub typ: Type,
    pub is_mutable: bool,
    pub is_used: bool,
    pub is_pointer: bool,
    pub source_location: Option<Location>,
    pub transpile_name: String,
}
#[derive(Clone, Debug)]
pub struct FunctionInfo {
    pub name: String,
    pub return_type: Option<Type>,
    pub parameters: Vec<Parameter>,
    pub body: Option<Vec<Expr>>,
    pub scope_level: usize,
    pub is_public: bool,
    pub parent: Option<String>,
}

pub struct ValidatorContext {
    pub scopes: Vec<HashMap<String, Symbol>>,

    pub functions: HashMap<String, FunctionInfo>,

    pub struct_defs: HashMap<
        String,
        (
            Vec<(String, Type)>,
            Vec<(String, Vec<Parameter>, Vec<Expr>, Option<Type>)>,
        ),
    >,

    pub enum_defs: HashMap<String, Vec<String>>,
    pub current_function: Option<String>,
    pub current_struct: Option<String>,
    pub current_return: Option<Expr>,
}
impl Default for ValidatorContext {
    fn default() -> Self {
        Self::new()
    }
}
impl ValidatorContext {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            functions: HashMap::new(),
            struct_defs: HashMap::new(),
            enum_defs: HashMap::new(),
            current_struct: None,
            current_function: None,
            current_return: None,
        }
    }
    pub fn lookup_variable_scoped(&self, name: &str) -> Option<(usize, Symbol)> {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if let Some(symbol) = scope.get(name) {
                return Some((self.scopes.len() - 1 - i, symbol.clone()));
            }
        }
        None
    }

    pub fn declare_variable(&mut self, name: String, symbol: Symbol) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, symbol);
        }
    }

    pub fn declare_function(&mut self, func: FunctionInfo) {
        self.functions.insert(func.name.clone(), func);
    }

    pub fn lookup_function(&self, name: &str) -> Option<FunctionInfo> {
        self.functions.get(name).cloned()
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }
    pub fn update_function_body_and_params(
        &mut self,
        name: &str,
        params: Vec<Parameter>,
        body: Vec<Expr>,
    ) {
        if let Some(func) = self.functions.get_mut(name) {
            func.parameters = params;
            func.body = Some(body);
        }
    }
}
