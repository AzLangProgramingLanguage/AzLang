use std::{collections::HashMap, rc::Rc, vec};
pub mod ast;
pub mod decl;
pub mod errors;
pub mod expr;
mod helper;
mod tests;
pub mod validate;
use crate::{
    ast::{Function, Program},
    errors::ValidatorError,
    validate::{ValidatorExpr, validate_statement},
};
use parser::{
    ast::{Atom, Parameter, Statement, Symbol},
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
    pub variables: Vec<HashMap<Atom, Symbol>>,
    pub link_files: Vec<String>,
    pub enums: HashMap<String, Vec<String>>,
}
pub type ValidatedProgram = Result<(Validator, Program), ValidatorError>;
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
                    ..
                } => {
                    let namestr = name.to_string();
                    if let Some(_) = self.functions.get(&namestr) {
                        return Err(ValidatorError::FunctionAlreadyDefined(namestr));
                    }
                    self.functions.insert(
                        namestr,
                        FunctionInfo {
                            return_type: return_typ.clone(),
                            parameters: params.clone(),
                        },
                    );
                }
                Statement::ExternalFunctionDef {
                    name,
                    return_typ,
                    params,
                    library,
                } => {
                    let namestr = name.to_string();
                    if let Some(_) = self.functions.get(&namestr) {
                        return Err(ValidatorError::FunctionAlreadyDefined(namestr));
                    }

                    self.link_files.push(library.to_string());
                    self.functions.insert(
                        namestr,
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

    pub fn validate(mut self, ast: Vec<Statement>) -> ValidatedProgram {
        let mut program = Program {
            functions: vec![],
            expressions: vec![],
        };
        self.variables.push(HashMap::new());
        self.function_decl(&ast)?;
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
                    for param in params.clone() {
                        self.declare_variable(
                            param.name,
                            Symbol {
                                typ: param.typ,
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
                        name: name.to_string(),
                        body: validated_body,
                        params,
                        return_typ,
                    });
                }

                Statement::EnumDecl { name, variants } => {
                    for (i, v) in variants.into_iter().enumerate() {
                        self.declare_variable(
                            v.clone(),
                            Symbol {
                                typ: Type::User(name.clone()),
                                is_used: false,
                                is_mutable: false,
                                is_changed: false,
                            },
                        );
                        program.expressions.push(ast::Ast::Decl {
                            name: v.to_string(),
                            typ: Type::Natural,
                            is_mutable: false,
                            value: Box::new(ValidatorExpr::Number(i as i64)),
                        });
                    }
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
                if !symbol.is_used && !matches!(symbol.typ, Type::User(_)) {
                    return Err(ValidatorError::NotUsedVariable(name.to_string()));
                }
                if symbol.is_mutable && !symbol.is_changed {
                    return Err(ValidatorError::NeverChangedMuttableVariable(
                        name.to_string(),
                    ));
                }
            }
        }

        Ok((self, program))
    }
}
