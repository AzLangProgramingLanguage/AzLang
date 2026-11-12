use std::{collections::HashMap, rc::Rc, vec};
pub mod ast;
pub mod decl;
pub mod errors;
pub mod expr;
mod helper;
mod tests;
// pub mod validate;
use crate::{
    ast::{Expr, Program},
    decl::{validate_assignment, validate_decl},
    errors::ValidatorError,
    // validate::ValidatorExpr,
};
use parser::{
    ast::{Atom, Parameter, Statement},
    shared_ast::Type,
};

#[derive(Debug, PartialEq)]
pub struct FunctionInfo {
    pub return_type: Type,
    pub parameters: Vec<Parameter>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Symbol {
    pub typ: Type,
    pub is_mutable: bool,
    pub is_used: bool,
    pub is_changed: bool,
    pub val: Rc<Box<Expr>>,
}

#[derive(Debug)]
pub struct MethodInfo {
    pub name: String,
    pub return_type: Option<Type>,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Default, PartialEq)]
pub struct Validator {
    pub functions: HashMap<String, FunctionInfo>,
    pub variables: Vec<HashMap<Atom, Symbol>>,
    pub link_files: Vec<String>,
    pub enums: HashMap<String, Vec<String>>,
    // pub constvars: HashMap<String, ValidatorExpr>,
}
pub type ValidatedProgram<'a> = (&'a mut Validator, Program);
impl Validator {
    pub fn lookup_variable_mut_with_err(
        &mut self,
        var_name: &Atom,
    ) -> Result<&mut Symbol, ValidatorError> {
        for stack in self.variables.iter_mut().rev() {
            if let Some(symbol) = stack.get_mut(var_name) {
                return Ok(symbol);
            }
        }
        Err(ValidatorError::UndefinedVariable(var_name.to_string()))
    }
    pub fn lookup_variable(&self, var_name: &Atom) -> Option<&Symbol> {
        for stack in self.variables.iter().rev() {
            if let Some(symbol) = stack.get(var_name) {
                return Some(symbol);
            }
        }
        None
    }

    pub fn declare_variable(&mut self, var_name: Atom, symbol: Symbol) {
        if let Some(stack) = self.variables.last_mut() {
            stack.insert(var_name, symbol);
        }
    }

    pub fn validate(&mut self, ast: Vec<Statement>) -> Result<Program, ValidatorError> {
        let mut program = Program {
            functions: vec![],
            expressions: vec![],
            variables: HashMap::new(), //Double Byte
        };
        for stmt in ast {
            match stmt {
                Statement::Decl {
                    name,
                    typ,
                    is_mutable,
                    value,
                } => {
                    validate_decl(name, typ, is_mutable, value, self)?;
                }
                Statement::FunctionDef {
                    name,
                    return_typ,
                    params,
                    body,
                } => {}
                Statement::Assignment { name, value } => {
                    validate_assignment(self, name, value)?;
                }
                other => {
                    todo!("{other:?} isn't implemented yet")
                }
            }
        }

        if let Some(_) = self.variables.last() {
            // for (name, symbol) in scope {
            //     if !symbol.is_used && !matches!(symbol.typ, Type::User(_)) {
            //         return Err(ValidatorError::NotUsedVariable(name.to_string()));
            //     }
            //     if symbol.is_mutable && !symbol.is_changed {
            //         return Err(ValidatorError::NeverChangedMuttableVariable(
            //             name.to_string(),
            //         ));
            //     }
            // }
        }

        Ok(program)
    }
}
