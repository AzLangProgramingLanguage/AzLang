use std::collections::HashMap;
pub mod errors;
pub mod function_call;
mod helper;
mod typed_ast;
pub mod validate;
use crate::{errors::ValidatorError, validate::validate_expr};
use parser::{
    ast::{Expr, Parameter, Program, Symbol},
    shared_ast::Type,
};

#[derive(Debug)]
pub struct FunctionInfo {
    pub return_type: Option<Type>,
    pub parameters: Vec<Parameter>,
    pub variables: HashMap<String, Symbol>,
}

#[derive(Debug)]
pub struct MethodInfo {
    pub name: String,
    pub return_type: Option<Type>,
    pub parameters: Vec<Parameter>,
    pub is_allocator_used: bool,
}

#[derive(Debug)]
pub struct Validator {
    pub global_variables: HashMap<String, Symbol>,
    pub functions: HashMap<String, FunctionInfo>,
    pub struct_defs: HashMap<String, (Vec<(String, Type)>, Vec<MethodInfo>)>,
    pub union_defs: HashMap<String, (Vec<(String, Type)>, Vec<MethodInfo>)>,
    pub enum_defs: HashMap<String, Vec<String>>,
    pub is_allocator_used: bool,
    pub current_function: Option<String>,
    pub current_return: Option<Box<Expr>>,
    pub current_struct: Option<String>,
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

impl Validator {
    pub fn new() -> Self {
        Self {
            global_variables: HashMap::new(),
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

    pub fn validate_user_type(&self, name: &str) -> Result<(), ValidatorError> {
        if self.enum_defs.get(name).is_some() {
            return Ok(());
        }
        if self.struct_defs.get(name).is_some() {
            return Ok(());
        }
        if self.union_defs.get(name).is_some() {
            return Ok(());
        }
        Err(ValidatorError::UnknownType(name.to_string()))
    }

    pub fn lookup_variable(&mut self, name: &str) -> Option<&mut Symbol> {
        if let Some(function) = &self.current_function {
            self.functions
                .get_mut(function)
                .unwrap()
                .variables
                .get_mut(name)
        } else {
            self.global_variables.get_mut(name)
        }
    }

    pub fn declare_function(&mut self, name: String, func: FunctionInfo) -> Option<FunctionInfo> {
        self.functions.insert(name, func)
    }

    pub fn declare_variable(&mut self, name: String, variable: Symbol) {
        if let Some(function) = &self.current_function {
            self.functions
                .get_mut(function)
                .unwrap()
                .variables
                .insert(name, variable);
        } else {
            self.global_variables.insert(name, variable);
        }
    }

    pub fn validate(&mut self, parsed_program: &mut Program) -> Result<(), ValidatorError> {
        for expr in parsed_program.expressions.iter_mut() {
            validate_expr(expr, self)?;

        }
        for variable in &self.global_variables {
            if variable.1.is_mutable && !variable.1.is_changed {
                return Err(ValidatorError::NeverChangedMuttableVariable(
                    variable.0.to_string(),
                ));
            }
        }
        Ok(())
    }
}
