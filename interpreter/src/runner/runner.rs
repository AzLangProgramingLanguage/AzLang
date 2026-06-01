use core::fmt;
use std::fmt::Display;

use super::Runner;
use crate::runner::{
    Variable, binary_op::binary_op_runner, builtin::builthin_call_runner,
    function_call::function_call, helpers::run_body,
};

use parser::shared_ast::{BuiltInFunction, Type};
use validator::ast::TemplateChunk;
use validator::ast::{Ast, Expr};
type Statement = Ast;
#[derive(Debug, Clone, PartialEq)]
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

            Value::List(l) => {
                write!(f, "[")?;

                for (i, item) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "{}", item)?;
                }

                write!(f, "]")
            }

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
        Expr::VariableRef { name, symbol: _ } => {
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
        } => function_call(ctx, target, name, args, Some(returned_type)),
        Expr::BuiltInCall {
            function,
            args,
            return_type: _,
        } => match function {
            BuiltInFunction::Len => match args.first() {
                Some(current_expr) => {
                    let value = get_primitive_value(ctx, current_expr.clone(), cast_typ);
                    match value {
                        Value::List(s) => Value::Number(s.len() as i64),
                        Value::String(s) => Value::Number(s.len() as i64),
                        _ => Value::Number(0),
                    }
                }
                _ => Value::Void, // Some(Expr::List(s)) => Value::Number(s.len() as i64),
                                  // Some(Expr::String(s)) => Value::Number(s.len() as i64),
                                  // _ => {
                                  //     println!("{function:?}  {:?} ", args);
                                  //     Value::Number(0)
                                  // }
            },

            _ => Value::Void,
        },
        other => panic!("{other:#?} Invalid expression"),
    }
}

pub fn runner_interpretator(ctx: &mut Runner, stmt: Ast) {
    match stmt {
        Statement::Decl {
            name,
            typ,
            is_mutable,
            value,
        } => {
            let new_value: Value = get_primitive_value(ctx, *value, Some(typ.clone()));
            ctx.variables.insert(
                name.to_string(),
                Variable {
                    value: new_value,
                    // typ,
                    // is_mutable,
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
    }
}
