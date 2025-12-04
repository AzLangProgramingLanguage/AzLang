use std::{borrow::Cow, collections::HashMap};

pub mod errors;
mod helper;
pub mod validate;
use parser::{
    ast::{Expr, Parameter, Symbol},
    shared_ast::Type,
};

use crate::errors::ValidatorError;

#[derive(Debug)]
pub struct FunctionInfo<'a> {
    pub name: Cow<'a, str>,
    pub return_type: Option<Type<'a>>,
    pub parameters: Vec<Parameter<'a>>,
}

#[derive(Debug)]
pub struct MethodInfo<'a> {
    pub name: Cow<'a, str>,
    pub return_type: Option<Type<'a>>,
    pub parameters: Vec<Parameter<'a>>,
    pub is_allocator_used: bool,
}
#[derive(Debug)]
pub struct ValidatorContext<'a> {
    pub scopes: Vec<HashMap<String, Symbol<'a>>>,
    pub functions: HashMap<Cow<'a, str>, FunctionInfo<'a>>,
    pub struct_defs: HashMap<String, (Vec<(&'a str, Type<'a>)>, Vec<MethodInfo<'a>>)>,
    pub union_defs: HashMap<String, (Vec<(&'a str, Type<'a>)>, Vec<MethodInfo<'a>>)>,
    pub enum_defs: HashMap<Cow<'a, str>, Vec<Cow<'a, str>>>,
    pub is_allocator_used: bool,
    pub current_function: Option<String>,
    pub current_return: Option<Box<Expr<'a>>>,
    pub current_struct: Option<&'a str>,
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
            union_defs: HashMap::new(),
            is_allocator_used: false,
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
    pub fn validate_user_type(&self, name: &str) -> Result<(), ValidatorError> {
        if let Some(_) = self.enum_defs.get(name) {
            return Ok(());
        }
        if let Some(_) = self.struct_defs.get(name) {
            return Ok(());
        }
        if let Some(_) = self.union_defs.get(name) {
            return Ok(());
        }
        Err(ValidatorError::UnknownType(name.to_string()))
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

    pub fn lookup_variable_mut(&mut self, name: &str) -> Option<&mut Symbol<'a>> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(sym) = scope.get_mut(name) {
                return Some(sym);
            }
        }
        None
    }

    pub fn declare_function(&mut self, func: FunctionInfo<'a>) {
        self.functions.insert(func.name.clone(), func);
    }
}
