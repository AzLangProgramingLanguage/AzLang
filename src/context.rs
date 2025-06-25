use crate::parser::{Expr, ast::Type};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct Location {
    pub line: usize,
    pub column: usize,
    pub file: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Parameter {
    pub name: String,
    pub typ: Type,
    pub is_mutable: bool,
    pub is_pointer: bool,
}

#[derive(Clone, Debug)]
pub struct FunctionInfo {
    pub name: String,
    pub return_type: Option<Type>,
    pub parameters: Vec<Parameter>,
    pub body: Option<Vec<Expr>>,
    pub scope_level: usize,
    pub is_public: bool,
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub typ: Type,
    pub is_mutable: bool,
    pub is_used: bool,
    pub is_pointer: bool,
    pub source_location: Option<Location>,
}
#[derive(Clone, Debug)]

pub struct TranspileContext {
    pub imports: HashSet<String>,
    pub symbol_types: HashMap<String, Symbol>,
    pub scopes: Vec<HashMap<String, Symbol>>,
    pub struct_defs: HashMap<
        String,
        (
            Vec<(String, Type)>,
            Vec<(String, Vec<Parameter>, Vec<Expr>, Option<Type>)>,
        ),
    >,
    pub enum_defs: HashMap<String, Vec<String>>,
    pub current_struct: Option<String>,
    pub functions: HashMap<String, FunctionInfo>,
    pub needs_allocator: bool,
    pub uses_stdout: bool,
    pub used_input_fn: bool,
    pub cleanup_statements: Vec<String>,
    pub used_sum_fn: bool,
    pub used_split_n_fn: bool,
    pub used_split_auto_fn: bool,
    pub used_split_alloc_fn: bool,
}

impl TranspileContext {
    pub fn new() -> Self {
        Self {
            imports: HashSet::new(),
            symbol_types: HashMap::new(),
            scopes: vec![HashMap::new()],
            struct_defs: HashMap::new(),
            enum_defs: HashMap::new(),
            current_struct: None,
            functions: HashMap::new(),
            needs_allocator: false,
            uses_stdout: false,
            used_input_fn: false,
            used_split_alloc_fn: false,
            cleanup_statements: Vec::new(),
            used_sum_fn: false,
            used_split_n_fn: false,
            used_split_auto_fn: false,
        }
    }
    pub fn lookup_variable_scoped(&self, name: &str) -> Option<(usize, Symbol)> {
        for (level, scope) in self.scopes.iter().rev().enumerate() {
            if let Some(symbol) = scope.get(name) {
                return Some((self.scopes.len() - 1 - level, symbol.clone()));
            }
        }
        None
    }

    pub fn declare_variable(&mut self, name: String, symbol: Symbol) {
        self.symbol_types.insert(name.clone(), symbol.clone());

        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, symbol); // ✅ həm adı, həm də symbol'u daxil et
        }
    }

    pub fn declare_function(&mut self, func: FunctionInfo) {
        self.functions.insert(func.name.clone(), func);
    }

    pub fn lookup_function(&self, name: &str) -> Option<FunctionInfo> {
        self.functions.get(name).cloned()
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        if let Some(scope) = self.scopes.pop() {
            for (name, _) in scope {
                self.symbol_types.remove(&name);
            }
        }
    }
    pub fn update_function_body_and_params(
        &mut self,
        name: &str,
        params: Vec<Parameter>,
        body: Vec<Expr>,
    ) {
        if let Some(func) = self.functions.get_mut(name) {
            func.parameters = params;
            func.body = Some(body);
        }
    }

    pub fn add_import(&mut self, import: &str) -> Option<String> {
        if self.imports.contains(import) {
            None
        } else {
            self.imports.insert(import.to_string());
            Some(import.to_string())
        }
    }
}
