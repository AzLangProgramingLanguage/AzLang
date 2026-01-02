use std::{borrow::Cow, collections::HashMap};

pub mod errors;
mod helper;
pub mod validate;
use parser::{
    ast::{Expr, Parameter, Program, Symbol},
    shared_ast::Type,
};

use crate::{errors::ValidatorError, validate::validate_expr};

#[derive(Debug)]
pub struct FunctionInfo<'a> {
    pub return_type: Option<Type<'a>>,
    pub parameters: Vec<Parameter<'a>>,
    pub variables: HashMap<String, Symbol<'a>>,
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
    pub global_variables: HashMap<String, Symbol<'a>>,
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

    pub fn lookup_variable(&mut self, name: &str) -> Option<&mut Symbol<'a>> {
        if let Some(function) = &self.current_function {
            self.functions
                .get_mut(&Cow::Owned(function.to_string()))
                .unwrap()
                .variables
                .get_mut(name)
        } else {
            self.global_variables.get_mut(name)
        }
    }

    pub fn declare_function(
        &mut self,
        name: Cow<'a, str>,
        func: FunctionInfo<'a>,
    ) -> Option<FunctionInfo<'a>> {
        self.functions.insert(name, func)
    }
    pub fn declare_variable(&mut self, name: String, variable: Symbol<'a>) {
        if let Some(function) = &self.current_function {
            self.functions
                .get_mut(&Cow::Owned(function.to_string()))
                .unwrap()
                .variables
                .insert(name, variable);
        } else {
            self.global_variables.insert(name, variable);
        }
    }
    pub fn validate(&mut self, parsed_program: &mut Program<'a>) -> Result<(), ValidatorError> {
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
