use std::ffi::{CStr, CString};

use libloading::{Library, Symbol};
use parser::shared_ast::Type;

use validator::ast::{Ast, Expr};

use crate::runner::{
    Runner, Variable,
    runner::{Value, get_primitive_value, runner_interpretator},
};

use super::ExternalFunction;

pub fn function_call(
    ctx: &mut Runner,
    _target: Option<Box<Expr>>,
    name: Box<Expr>,
    args: Vec<Expr>,
    _returned_type: Option<Type>,
) -> Value {
    match *name {
        Expr::VariableRef { name, symbol: _ } => {
            if let Some(function) = ctx.functions.get(&name).cloned() {
                for (index, param) in function.params.iter().enumerate() {
                    let variable = get_primitive_value(ctx, args[index].clone(), None);
                    ctx.variables.insert(
                        param.name.clone(),
                        Variable {
                            value: variable,
                        },
                    );
                }
                for stmt in function.body.clone() {
                    match stmt {
                        Ast::Expr(Expr::Return(e)) => {
                            return get_primitive_value(ctx, *e, Some(function.return_type));
                        }
                        _ => runner_interpretator(ctx, stmt),
                    }
                }
                Value::Void
            } else if let Some(ext) = ctx.external_functions.get(&name).cloned() {
                let arg_values: Vec<Value> = args
                    .iter()
                    .map(|a| get_primitive_value(ctx, a.clone(), None))
                    .collect();
                call_external_fn(&ext, &arg_values)
            } else {
                panic!("{name} function not found")
            }
        }
        other => todo!("{other:?} not implemented yet"),
    }
}

fn call_external_fn(ext: &ExternalFunction, args: &[Value]) -> Value {
    let lib = unsafe { Library::new(&ext.library) }
        .unwrap_or_else(|e| panic!("Failed to load library '{}': {}", ext.library, e));

    let symbol_bytes = ext.symbol.as_bytes();
    let param_count = ext.params.len();
    let void_ret = ext.return_type == Type::Void;

    match (param_count, void_ret) {
        (0, true) => {
            let f: Symbol<unsafe extern "C" fn()> =
                unsafe { lib.get(symbol_bytes) }
                    .unwrap_or_else(|e| panic!("Symbol '{}' not found: {}", ext.symbol, e));
            unsafe { f() };
            Value::Void
        }
        (0, false) => {
            let f: Symbol<unsafe extern "C" fn() -> i64> =
                unsafe { lib.get(symbol_bytes) }
                    .unwrap_or_else(|e| panic!("Symbol '{}' not found: {}", ext.symbol, e));
            Value::Number(unsafe { f() })
        }
        (1, true) => {
            let param_type = &ext.params[0].typ;
            match param_type {
                Type::Integer | Type::Natural | Type::BigInteger | Type::LowInteger => {
                    let f: Symbol<unsafe extern "C" fn(i64)> =
                        unsafe { lib.get(symbol_bytes) }
                            .unwrap_or_else(|e| panic!("Symbol '{}' not found: {}", ext.symbol, e));
                    unsafe { f(args[0].as_number()) };
                    Value::Void
                }
                Type::Float => {
                    let f: Symbol<unsafe extern "C" fn(f64)> =
                        unsafe { lib.get(symbol_bytes) }
                            .unwrap_or_else(|e| panic!("Symbol '{}' not found: {}", ext.symbol, e));
                    unsafe { f(args[0].as_float()) };
                    Value::Void
                }
                Type::String(_) => {
                    let f: Symbol<unsafe extern "C" fn(*const std::ffi::c_char)> =
                        unsafe { lib.get(symbol_bytes) }
                            .unwrap_or_else(|e| panic!("Symbol '{}' not found: {}", ext.symbol, e));
                    let cstr = CString::new(args[0].as_string()).unwrap();
                    unsafe { f(cstr.as_ptr()) };
                    Value::Void
                }
                Type::Bool => {
                    let f: Symbol<unsafe extern "C" fn(u8)> =
                        unsafe { lib.get(symbol_bytes) }
                            .unwrap_or_else(|e| panic!("Symbol '{}' not found: {}", ext.symbol, e));
                    unsafe { f(args[0].as_bool() as u8) };
                    Value::Void
                }
                _ => panic!("Unsupported FFI param type: {:?}", param_type),
            }
        }
        (1, false) => {
            let param_type = &ext.params[0].typ;
            match param_type {
                Type::Integer | Type::Natural | Type::BigInteger | Type::LowInteger => {
                    let f: Symbol<unsafe extern "C" fn(i64) -> i64> =
                        unsafe { lib.get(symbol_bytes) }
                            .unwrap_or_else(|e| panic!("Symbol '{}' not found: {}", ext.symbol, e));
                    Value::Number(unsafe { f(args[0].as_number()) })
                }
                Type::Float => {
                    let f: Symbol<unsafe extern "C" fn(f64) -> f64> =
                        unsafe { lib.get(symbol_bytes) }
                            .unwrap_or_else(|e| panic!("Symbol '{}' not found: {}", ext.symbol, e));
                    Value::Float(unsafe { f(args[0].as_float()) })
                }
                Type::String(_) => {
                    let f: Symbol<unsafe extern "C" fn(*const std::ffi::c_char) -> *const std::ffi::c_char> =
                        unsafe { lib.get(symbol_bytes) }
                            .unwrap_or_else(|e| panic!("Symbol '{}' not found: {}", ext.symbol, e));
                    let cstr = CString::new(args[0].as_string()).unwrap();
                    let ret = unsafe { f(cstr.as_ptr()) };
                    let ret_str = unsafe { CStr::from_ptr(ret).to_string_lossy().into_owned() };
                    Value::String(ret_str)
                }
                _ => panic!("Unsupported FFI param type: {:?}", param_type),
            }
        }
        (2, true) => {
            let t0 = &ext.params[0].typ;
            let t1 = &ext.params[1].typ;
            match (t0, t1) {
                (Type::Integer | Type::Natural | Type::BigInteger | Type::LowInteger,
                 Type::Integer | Type::Natural | Type::BigInteger | Type::LowInteger) => {
                    let f: Symbol<unsafe extern "C" fn(i64, i64)> =
                        unsafe { lib.get(symbol_bytes) }
                            .unwrap_or_else(|e| panic!("Symbol '{}' not found: {}", ext.symbol, e));
                    unsafe { f(args[0].as_number(), args[1].as_number()) };
                    Value::Void
                }
                (Type::String(_), Type::String(_)) => {
                    let f: Symbol<unsafe extern "C" fn(*const std::ffi::c_char, *const std::ffi::c_char)> =
                        unsafe { lib.get(symbol_bytes) }
                            .unwrap_or_else(|e| panic!("Symbol '{}' not found: {}", ext.symbol, e));
                    let cstr0 = CString::new(args[0].as_string()).unwrap();
                    let cstr1 = CString::new(args[1].as_string()).unwrap();
                    unsafe { f(cstr0.as_ptr(), cstr1.as_ptr()) };
                    Value::Void
                }
                _ => panic!("Unsupported FFI param types: {:?}, {:?}", t0, t1),
            }
        }
        (n, _) => panic!("FFI with {n} params not yet supported (max 2)"),
    }
}

trait ValueExt {
    fn as_string(&self) -> String;
    fn as_bool(&self) -> bool;
}

impl ValueExt for Value {
    fn as_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            _ => format!("{}", self),
        }
    }
    fn as_bool(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0,
            _ => false,
        }
    }
}
