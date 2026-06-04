use std::{collections::HashMap, vec};
pub mod ast;
pub mod errors;
pub mod expr;
mod helper;
mod tests;
pub mod validate;
use crate::{
    ast::{Function, Program},
    errors::ValidatorError,
    validate::validate_statement,
};
use parser::{
    ast::{Expr, FunctionDef, Parameter, Statement, Symbol},
    shared_ast::Type,
};

#[derive(Debug, PartialEq)]
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

#[derive(Debug, Default, PartialEq)]
pub struct Validator {
    pub functions: HashMap<String, FunctionInfo>,
    pub variables: Vec<HashMap<String, Symbol>>,
}

impl Validator {
    pub fn function_decl(&mut self, ast: &Vec<Statement>) -> &mut Validator {
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
        self
    }
    pub fn lookup_variable_mut_with_err(
        &mut self,
        var_name: &str,
    ) -> Result<&mut Symbol, ValidatorError> {
        for stack in self.variables.iter_mut().rev() {
            if let Some(symbol) = stack.get_mut(var_name) {
                return Ok(symbol);
            }
        }
        Err(ValidatorError::UndefinedVariable(var_name.to_string()))
    }
    pub fn lookup_variable(&self, var_name: &str) -> Option<&Symbol> {
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

    pub fn validate(mut self, ast: Vec<Statement>) -> Result<(Validator, Program), ValidatorError> {
        let mut program = Program {
            functions: vec![],
            expressions: vec![],
        };
        self.variables.push(HashMap::new());
        self.function_decl(&ast);
        for stmt in ast {
            match stmt {
                Statement::FunctionDef {
                    name,
                    return_typ,
                    params,
                    body,
                } => {
                    let mut validated_body = Vec::new();
                    self.variables.push(HashMap::new());
                    for param in &params {
                        self.declare_variable(
                            param.name.clone(),
                            Symbol {
                                typ: param.typ.clone(),
                                is_mutable: param.is_pointer,
                                is_used: false,
                                is_changed: false,
                            },
                        );
                    }
                    for s in body {
                        validated_body.push(validate_statement(s, &mut self)?);
                    }
                    self.variables.pop();
                    program.functions.push(Function {
                        name,
                        body: validated_body,
                        params,
                        return_typ,
                    });
                }
                stmt => {
                    program
                        .expressions
                        .push(validate_statement(stmt, &mut self)?);
                }
            }
        }

        if let Some(scope) = self.variables.last() {
            for (name, symbol) in scope {
                if !symbol.is_used {
                    return Err(ValidatorError::NotUsedVariable(name.clone()));
                }
                if symbol.is_mutable && !symbol.is_changed {
                    return Err(ValidatorError::NeverChangedMuttableVariable(name.clone()));
                }
            }
        }

        Ok((self, program))
    }
}
