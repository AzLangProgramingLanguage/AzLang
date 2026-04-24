use core::fmt;
use std::{fmt::Display, rc::Rc};

use super::Runner;
use crate::runner::{
    Variable, binary_op::binary_op_runner, builtin::builthin_call_runner, helpers::run_body,
};

use parser::{
    ast::{Expr, Statement, Symbol, TemplateChunk},
    shared_ast::Type,
};
#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Char(char),
    List(Vec<Value>),
    Void,
}
impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Char(c) => write!(f, "{}", c),
            Value::List(l) => write!(f, "{:?}", l),
            Value::Void => write!(f, "void"),
        }
    }
}
impl Value {
    pub fn as_number(&self) -> i64 {
        match self {
            Value::Number(n) => *n,
            _ => 0,
        }
    }
    pub fn as_float(&self) -> f64 {
        match self {
            Value::Float(f) => *f,
            _ => 0.0,
        }
    }
}

pub fn get_primitive_value(ctx: &mut Runner, expr: Expr, cast_typ: Option<Type>) -> Value {
    match expr {
        Expr::Number(n) => Value::Number(n),
        Expr::String(s) => Value::String(s),
        Expr::Float(f) => Value::Float(f),
        Expr::Bool(b) => Value::Bool(b),
        Expr::Char(c) => Value::Char(c),
        Expr::TemplateString(chunks) => {
            let mut s = String::new();
            for chunk in chunks {
                match chunk {
                    TemplateChunk::Literal(l) => s.push_str(&l),
                    TemplateChunk::Expr(v) => {
                        s.push_str(&get_primitive_value(ctx, *v, None).to_string())
                    }
                }
            }
            Value::String(s)
        }
        Expr::List(l) => Value::List(
            l.iter()
                .map(|x| get_primitive_value(ctx, x.clone(), None))
                .collect(),
        ),
        Expr::Void => Value::Void,
        Expr::VariableRef { name, symbol } => {
            let var = ctx.variables.get(&name).unwrap();
            var.value.clone()
        }
        Expr::BinaryOp {
            left,
            right,
            op,
            return_type,
        } => {
            let left_value = get_primitive_value(ctx, *left, None);
            let right_value = get_primitive_value(ctx, *right, None);
            binary_op_runner(ctx, left_value, right_value, op, Some(return_type))
        }
        Expr::Call {
            target,
            name,
            args,
            returned_type,
        } => match *name {
            Expr::VariableRef {
                name,
                symbol:
                    Some(Symbol {
                        typ: Type::Function,
                        ..
                    }),
            } => {
                if let Some(function) = ctx.functions.get(&name).cloned() {
                    for (index, param) in function.params.iter().enumerate() {
                        let variable = get_primitive_value(ctx, args[index].clone(), None);
                        ctx.variables.insert(
                            param.name.clone(),
                            Variable {
                                value: variable,
                                typ: Rc::new(param.typ.clone()),
                                is_mutable: param.is_mutable,
                            },
                        );
                    }
                    for stmt in function.body.clone() {
                        runner_interpretator(ctx, stmt);
                    }
                    Value::Void
                } else {
                    panic!("{name} function not found")
                }
            }
            _ => todo!(),
        },
        other => panic!("{other:#?} Invalid expression"),
    }
}

pub fn runner_interpretator(ctx: &mut Runner, stmt: Statement) {
    match stmt {
        Statement::Decl {
            name,
            typ,
            is_mutable,
            value,
        } => {
            let new_value: Value = get_primitive_value(ctx, *value, Some((*typ).clone()));
            ctx.variables.insert(
                name.to_string(),
                Variable {
                    value: new_value,
                    typ,
                    is_mutable,
                },
            );
        }
        Statement::Condition { main, elif, other } => {
            if matches!(
                get_primitive_value(ctx, *main.condition, None),
                Value::Bool(true)
            ) {
                return run_body(ctx, main.body);
            }
            for branch in elif {
                if matches!(
                    get_primitive_value(ctx, *branch.condition, None),
                    Value::Bool(true)
                ) {
                    return run_body(ctx, branch.body);
                }
            }
            if let Some(other) = other {
                run_body(ctx, other.body);
            }
        }
        Statement::Assignment { name, value, .. } => {
            let new_value: Value = get_primitive_value(ctx, *value, None);
            let var = ctx.variables.get_mut(&name).unwrap();
            var.value = new_value;
        }
        Statement::Expr(expr) => match expr {
            Expr::Return(v) => {
                ctx.current_return = *v;
            }
            Expr::BuiltInCall {
                function,
                args,
                return_type,
            } => {
                let args_values: Vec<Value> = args
                    .iter()
                    .map(|x| get_primitive_value(ctx, x.clone(), None))
                    .collect();

                builthin_call_runner(function, args_values, return_type);
            }
            other => todo!("{other:?} not yet implemented "),
        },
        _ => {} /*
                 Expr::Loop {
                     var_name,
                     iterable,
                     body,
                 } => {
                     let iterable_value = runner_interpretator(ctx, *iterable);
                     if let Expr::List(list) = iterable_value {
                         for item in list {
                             ctx.variables.insert(
                                 var_name.to_string(),
                                 Variable {
                                     value: Rc::new(item),
                                     typ: Rc::new(Type::Any),
                                     is_mutable: false,
                                 },
                             );
                             run_body(ctx, body.clone());
                         }
                     }
                     Expr::Void
                 }
                 Expr::Return(value) => {
                     ctx.current_return = runner_interpretator(ctx, *value);
                     Expr::Void
                 }
                 Expr::Call {
                     target,
                     name,
                     args,
                     returned_type,
                 } => function_call(ctx, target, name, args, returned_type),
                 Expr::Assignment { name, value, .. } => {
                     let new_value: Expr = runner_interpretator(ctx, *value);
                     if let Some(var) = ctx.variables.get_mut(&name.to_string()) {
                         var.value = Rc::new(new_value);
                     }

                     Expr::Void
                 }
                 Expr::Condition { main, elif, other } => {
                                          Expr::Void
                 }
                 Expr::Index {
                     target,
                     index,
                     target_type,
                 } => {
                     let new_target = runner_interpretator(ctx, *target);
                     let new_index = runner_interpretator(ctx, *index);
                     match (new_target, new_index) {
                         (Expr::List(s), Expr::Number(n)) => {
                             return s.get(n as usize).unwrap().clone(); //TODO: Uncessessary CLone
                         }
                         (Expr::String(s), Expr::Number(n)) => {
                             return Expr::Char(s.chars().nth(n as usize).unwrap()); /* TODO: Used unwrap
                             remove it, it's too
                             dangerious
                              */
                         }
                         _ => {}
                     }
                     Expr::Void
                 }
                 Expr::BinaryOp {
                     left,
                     right,
                     op,
                     return_type,
                 } => binary_op_runner(ctx, left, right, op, return_type),

                 Expr::BuiltInCall {
                     function,
                     args,
                     return_type,
                 } => builthin_call_runner(ctx, function, args, return_type),
                 Expr::VariableRef { name, symbol } => {
                     if let Some(var) = ctx.variables.get(&name) {
                         return var.value.as_ref().clone();
                     }

                     Expr::Void
                 }
                 other => other,
                */
    }
}
