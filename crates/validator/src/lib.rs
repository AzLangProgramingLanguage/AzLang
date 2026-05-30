use std::{collections::HashMap, vec};
mod ast;
pub mod errors;
pub mod expr;
pub mod function_call;
mod helper;
mod tests;
pub mod validate;
use crate::{ast::Program, errors::ValidatorError, validate::validate_statement};
use parser::{
    ast::{Expr, FunctionDef, Parameter, Statement, Symbol},
    shared_ast::Type,
};

#[derive(Debug)]
pub struct FunctionInfo {
    pub return_type: Type,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug)]
pub struct MethodInfo {
    pub name: String,
    pub return_type: Option<Type>,
    pub parameters: Vec<Parameter>,
    pub is_allocator_used: bool,
}

#[derive(Debug, Default)]
pub struct Validator {
    pub functions: HashMap<String, FunctionInfo>,
    pub variables: Vec<HashMap<String, Symbol>>,
}

impl Validator {
    pub fn function_decl(
        &mut self,
        ast: &Vec<Statement>,
    ) -> Result<&mut Validator, ValidatorError> {
        for stmt in ast {
            match stmt {
                Statement::FunctionDef {
                    name,
                    return_typ,
                    params,
                    body,
                } => {
                    self.functions.insert(
                        name.clone(),
                        FunctionInfo {
                            return_type: return_typ.clone(),
                            parameters: params.clone(),
                        },
                    );
                }
                _ => continue,
            }
        }
        Ok(self)
    }
    pub fn lookup_variable(&self, var_name: &String) -> Option<&Symbol> {
        for stack in self.variables.iter().rev() {
            if let Some(symbol) = stack.get(var_name) {
                return Some(symbol);
            }
        }
        None
    }
    pub fn declare_variable(&mut self, var_name: String, symbol: Symbol) {
        if let Some(stack) = self.variables.last_mut() {
            stack.insert(var_name, symbol);
        }
    }

    pub fn validate(mut self, ast: Vec<Statement>) -> Result<Validator, ValidatorError> {
        let mut program = Program {
            functions: vec![],
            expressions: vec![],
        };
        self.function_decl(&ast);
        for stmt in ast {
            validate_statement(stmt, &mut program, &mut self);
        }

        Ok(self)
    }

    // pub fn validate_user_type(&self, name: &str) -> Result<(), ValidatorError> {
    //     if self.enum_defs.get(name).is_some() {
    //         return Ok(());
    //     }
    //     if self.struct_defs.get(name).is_some() {
    //         return Ok(());
    //     }
    //     if self.union_defs.get(name).is_some() {
    //         return Ok(());
    //     }
    //     Err(ValidatorError::UnknownType(name.to_string()))
    // }

    // pub fn lookup_variable(&mut self, name: &str) -> Option<&mut Symbol> {
    //     // if let Some(function) = &self.current_function {
    //     //     self.functions
    //     //         .get_mut(function)
    //     //         .unwrap()
    //     //         .variables
    //     //         .get_mut(name)
    //     // } else {
    //     self.global_variables.get_mut(name)
    //     // }
    // }

    // pub fn declare_variable(&mut self, name: String, variable: Symbol) {
    //     // if let Some(function) = &self.current_function {
    //     //     self.functions
    //     //         .get_mut(function)
    //     //         .unwrap()
    //     //         .variables
    //     //         .insert(name, variable);
    //     // } else {
    //     self.global_variables.insert(name, variable);
    //     // }
    // }
    //
    // pub fn validate(&mut self, parsed_program: &mut Program) -> Result<(), ValidatorError> {
    //     for func in self.functions.iter_mut() {
    //         for param in &func.1.params {
    //             self.global_variables.insert(
    //                 param.name.clone(),
    //                 Symbol {
    //                     typ: param.typ.clone(),
    //                     is_mutable: param.is_mutable,
    //                     is_pointer: param.is_pointer,
    //                     is_used: false,
    //                     is_changed: false,
    //                 },
    //             );
    //         }
    //         for stmt in func.1.body.iter_mut() {
    //             validate_statement(stmt, self)?;
    //         }
    //         for param in &func.1.params {
    //             self.global_variables.remove(&param.name);
    //         }
    //     }
    //     for expr in parsed_program.expressions.iter_mut() {
    //         validate_statement(expr, self)?;
    //     }
    //     for variable in &self.global_variables {
    //         if variable.1.is_mutable && !variable.1.is_changed {
    //             return Err(ValidatorError::NeverChangedMuttableVariable(
    //                 variable.0.to_string(),
    //             ));
    //         }
    //         if !variable.1.is_used {
    //             return Err(ValidatorError::NotUsedVariable(variable.0.clone()));
    //         }
    //     }
    //     Ok(())
    // }
}
